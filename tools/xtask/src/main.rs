//! Repository-owned validation tasks.

use std::{
    env, fs,
    path::{Path, PathBuf},
    process,
};

const REQUIRED_ADR_FIELDS: &[&str] = &["- Status:", "- Date:", "- Decision owners:"];
const REQUIRED_ADR_SECTIONS: &[&str] = &[
    "## Context",
    "## Decision",
    "## Consequences",
    "## References",
];
const REQUIRED_SEM_SECTIONS: &[&str] = &[
    "## Topic",
    "## Status",
    "## Primary authority",
    "## Sources",
    "## Ferraxis behavior",
    "## Tests",
    "## Variance",
];

fn main() {
    let command = env::args().nth(1).unwrap_or_else(|| "help".to_owned());
    let result = match command.as_str() {
        "validate" => validate_all(),
        "validate-docs" => validate_docs(),
        _ => Err("usage: cargo run -p xtask -- <validate|validate-docs>".to_owned()),
    };

    if let Err(error) = result {
        eprintln!("xtask: {error}");
        process::exit(1);
    }
}

fn validate_all() -> Result<(), String> {
    validate_docs()?;
    println!("xtask: repository validation passed");
    Ok(())
}

fn validate_docs() -> Result<(), String> {
    let root = workspace_root()?;
    validate_adrs(&root)?;
    validate_milestones(&root)?;
    validate_semantics(&root)?;
    validate_required_text(&root)?;
    validate_workflow(&root)?;
    println!("xtask: documentation structure passed");
    Ok(())
}

fn workspace_root() -> Result<PathBuf, String> {
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    manifest
        .parent()
        .and_then(Path::parent)
        .map(Path::to_path_buf)
        .ok_or_else(|| "could not determine workspace root".to_owned())
}

fn validate_adrs(root: &Path) -> Result<(), String> {
    let dir = root.join("docs/adr");
    let mut ids = Vec::new();
    for entry in sorted_markdown_files(&dir)? {
        let name = entry
            .file_name()
            .and_then(|value| value.to_str())
            .unwrap_or_default();
        if matches!(name, "README.md" | "TEMPLATE.md") {
            continue;
        }
        let Some(id) = adr_id_from_filename(name) else {
            return Err(format!("invalid ADR filename: {}", entry.display()));
        };
        let text = read(&entry)?;
        require_contains(&entry, &text, &format!("# {id}:"))?;
        for field in REQUIRED_ADR_FIELDS {
            require_contains(&entry, &text, field)?;
        }
        for section in REQUIRED_ADR_SECTIONS {
            require_contains(&entry, &text, section)?;
        }
        ids.push(id);
    }
    ensure_unique("ADR", &ids)?;
    if ids.is_empty() {
        return Err("ADR registry contains no decision records".to_owned());
    }
    Ok(())
}

fn validate_milestones(root: &Path) -> Result<(), String> {
    let path = root.join("docs/MILESTONES.md");
    let text = read(&path)?;
    let mut ids = Vec::new();
    for word in text.split(|c: char| !(c.is_ascii_alphanumeric() || c == '-')) {
        if word.starts_with('P') && word.contains("-M") {
            if !valid_milestone_id(word) {
                return Err(format!(
                    "invalid milestone ID `{word}` in {}",
                    path.display()
                ));
            }
            ids.push(word.to_owned());
        }
    }
    ensure_unique("milestone", &ids)?;
    for required in ["P0-M001", "P0-M020", "P0-M022", "P1-M001"] {
        if !ids.iter().any(|id| id == required) {
            return Err(format!("required milestone `{required}` missing"));
        }
    }
    Ok(())
}

fn validate_semantics(root: &Path) -> Result<(), String> {
    let dir = root.join("docs/semantics");
    let mut ids = Vec::new();
    for entry in sorted_markdown_files(&dir)? {
        let name = entry
            .file_name()
            .and_then(|value| value.to_str())
            .unwrap_or_default();
        if matches!(name, "README.md" | "TEMPLATE.md") {
            continue;
        }
        let Some(id) = semantic_id_from_filename(name) else {
            return Err(format!(
                "invalid semantic record filename: {}",
                entry.display()
            ));
        };
        let text = read(&entry)?;
        require_contains(&entry, &text, &format!("# {id}:"))?;
        for section in REQUIRED_SEM_SECTIONS {
            require_contains(&entry, &text, section)?;
        }
        ids.push(id);
    }
    ensure_unique("semantic evidence", &ids)?;
    if ids.is_empty() {
        return Err("semantic evidence registry contains no records".to_owned());
    }
    Ok(())
}

fn validate_required_text(root: &Path) -> Result<(), String> {
    let authority_path = root.join("docs/SEMANTICS_AUTHORITY.md");
    let authority = read(&authority_path)?;
    for text in [
        "Explicit Rust language definition/specification",
        "Stable documented Rust behavior",
        "`rustc` observable behavior",
        "Differential tests",
        "Documented Ferraxis extension/variance",
    ] {
        require_contains(&authority_path, &authority, text)?;
    }

    let invariants_path = root.join("docs/COMPILER_INVARIANTS.md");
    let invariants = read(&invariants_path)?;
    for number in 1..=22 {
        require_contains(&invariants_path, &invariants, &format!("INV-{number:03}"))?;
    }

    let releases_path = root.join("docs/RELEASES.md");
    let releases = read(&releases_path)?;
    for version in ["`0.0.1`", "`0.1.0`", "`0.6.0`"] {
        require_contains(&releases_path, &releases, version)?;
    }
    Ok(())
}

fn validate_workflow(root: &Path) -> Result<(), String> {
    for relative in [
        "PROJECT_SPEC.md",
        "PROJECT_STATE.md",
        "AGENT_HANDOFF.md",
        ".plans/README.md",
        ".plans/TEMPLATE.plan.md",
        "docs/DEVELOPMENT_WORKFLOW.md",
    ] {
        let path = root.join(relative);
        if !path.is_file() {
            return Err(format!("required workflow file missing: {}", path.display()));
        }
        let text = read(&path)?;
        if !text.ends_with('\n') {
            return Err(format!("{} must end with a newline", path.display()));
        }
    }

    let agents_path = root.join("AGENTS.md");
    let agents = read(&agents_path)?;
    for required in [
        "## Plan-first workflow",
        "## Agent startup checklist",
        "## No-bypass rules",
        "## Recovery rules",
        "## Completion evidence",
    ] {
        require_contains(&agents_path, &agents, required)?;
    }

    let spec_path = root.join("PROJECT_SPEC.md");
    let spec = read(&spec_path)?;
    for required in [
        "## Mission",
        "## Non-goals",
        "## Semantic authority",
        "## Architecture contract",
        "## Work contract",
        "## Toolchain contract",
        "## Completion definition",
    ] {
        require_contains(&spec_path, &spec, required)?;
    }

    let plan_template_path = root.join(".plans/TEMPLATE.plan.md");
    let plan_template = read(&plan_template_path)?;
    for required in [
        "## Goal",
        "## Non-goals",
        "## Architecture placement",
        "## Invariants",
        "## ADRs",
        "## Semantic evidence",
        "## Test-first matrix",
        "## Failure modes",
        "## Quality gates",
        "## Acceptance criteria",
        "## Completion record",
    ] {
        require_contains(&plan_template_path, &plan_template, required)?;
    }

    let active_path = root.join(".plans/ACTIVE");
    if active_path.is_file() {
        let active = read(&active_path)?;
        let active = active.trim();
        if !active.starts_with(".plans/") || !active.ends_with(".plan.md") {
            return Err(format!("invalid active plan path: {active}"));
        }

        let plan_path = root.join(active);
        if !plan_path.is_file() {
            return Err(format!("active plan is missing: {}", plan_path.display()));
        }

        let plan = read(&plan_path)?;
        for required in [
            "# Plan:",
            "Status:",
            "Milestone:",
            "## Goal",
            "## Acceptance criteria",
            "## Completion record",
        ] {
            require_contains(&plan_path, &plan, required)?;
        }
    }

    Ok(())
}

fn sorted_markdown_files(dir: &Path) -> Result<Vec<PathBuf>, String> {
    let mut files = fs::read_dir(dir)
        .map_err(|error| format!("failed to read {}: {error}", dir.display()))?
        .map(|entry| entry.map(|value| value.path()))
        .collect::<Result<Vec<_>, _>>()
        .map_err(|error| format!("failed to read directory entry: {error}"))?;
    files.retain(|path| path.extension().and_then(|value| value.to_str()) == Some("md"));
    files.sort();
    Ok(files)
}

fn adr_id_from_filename(name: &str) -> Option<String> {
    let rest = name.strip_prefix("ADR-")?;
    let (digits, suffix) = rest.split_once('-')?;
    if digits.len() != 4 || !digits.chars().all(|c| c.is_ascii_digit()) || !suffix.ends_with(".md")
    {
        return None;
    }
    Some(format!("ADR-{digits}"))
}

fn semantic_id_from_filename(name: &str) -> Option<String> {
    let rest = name.strip_prefix("SEM-")?;
    let mut parts = rest.splitn(3, '-');
    let domain = parts.next()?;
    let digits = parts.next()?;
    let suffix = parts.next()?;
    if domain.len() != 3
        || !domain.chars().all(|c| c.is_ascii_uppercase())
        || digits.len() != 4
        || !digits.chars().all(|c| c.is_ascii_digit())
        || !suffix.ends_with(".md")
    {
        return None;
    }
    Some(format!("SEM-{domain}-{digits}"))
}

fn valid_milestone_id(id: &str) -> bool {
    let Some((phase, milestone)) = id.split_once("-M") else {
        return false;
    };
    let Some(phase_digits) = phase.strip_prefix('P') else {
        return false;
    };
    !phase_digits.is_empty()
        && phase_digits.chars().all(|c| c.is_ascii_digit())
        && milestone.len() == 3
        && milestone.chars().all(|c| c.is_ascii_digit())
}

fn ensure_unique(kind: &str, values: &[String]) -> Result<(), String> {
    let mut sorted = values.to_vec();
    sorted.sort();
    for pair in sorted.windows(2) {
        if pair[0] == pair[1] {
            return Err(format!("duplicate {kind} ID `{}`", pair[0]));
        }
    }
    Ok(())
}

fn read(path: &Path) -> Result<String, String> {
    fs::read_to_string(path).map_err(|error| format!("failed to read {}: {error}", path.display()))
}

fn require_contains(path: &Path, text: &str, needle: &str) -> Result<(), String> {
    if text.contains(needle) {
        Ok(())
    } else {
        Err(format!(
            "{} is missing required text `{needle}`",
            path.display()
        ))
    }
}
