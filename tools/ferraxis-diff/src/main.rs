//! Reproducible differential observations for Ferraxis.
//!
//! P0-M018 compares only acceptance at two documented boundaries: the current
//! Ferraxis lexer and a stable rustc macro token-tree probe. It does not claim
//! access to rustc's raw lexer stream or token-for-token equivalence.

use std::{
    collections::BTreeMap,
    env, fs,
    path::{Path, PathBuf},
    process::{self, Command},
};

use ferraxis_lexer::lex;
use ferraxis_source::SourceFile;

const CORPUS_DIR: &str = "tests/differential/lexer/cases";
const EXPECTATIONS_FILE: &str = "tests/differential/lexer/expectations.tsv";
const OUTPUT_DIR: &str = "target/differential/lexer";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Acceptance {
    Accept,
    Reject,
}

impl Acceptance {
    const fn as_str(self) -> &'static str {
        match self {
            Self::Accept => "accept",
            Self::Reject => "reject",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Classification {
    AgreeAccept,
    AgreeReject,
    FerraxisRejects,
    RustcRejects,
}

impl Classification {
    const fn as_str(self) -> &'static str {
        match self {
            Self::AgreeAccept => "agree_accept",
            Self::AgreeReject => "agree_reject",
            Self::FerraxisRejects => "ferraxis_rejects",
            Self::RustcRejects => "rustc_rejects",
        }
    }

    fn parse(value: &str) -> Result<Self, String> {
        match value {
            "agree_accept" => Ok(Self::AgreeAccept),
            "agree_reject" => Ok(Self::AgreeReject),
            "ferraxis_rejects" => Ok(Self::FerraxisRejects),
            "rustc_rejects" => Ok(Self::RustcRejects),
            _ => Err(format!("unknown classification `{value}`")),
        }
    }
}

#[derive(Debug)]
struct CaseResult {
    id: String,
    expected: Classification,
    observed: Classification,
    ferraxis: Acceptance,
    rustc: Acceptance,
    rustc_exit: String,
}

fn main() {
    if let Err(error) = run() {
        eprintln!("ferraxis-diff: {error}");
        process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let mut args = env::args().skip(1);
    match (args.next().as_deref(), args.next()) {
        (Some("lexer"), None) => run_lexer(),
        _ => Err("usage: cargo run -p ferraxis-diff --locked -- lexer".to_owned()),
    }
}

fn run_lexer() -> Result<(), String> {
    let root = workspace_root()?;
    let expectations = read_expectations(&root.join(EXPECTATIONS_FILE))?;
    let cases = discover_cases(&root.join(CORPUS_DIR))?;

    if cases.is_empty() {
        return Err("differential lexer corpus is empty".to_owned());
    }

    let discovered_ids = cases
        .iter()
        .map(|path| case_id(path))
        .collect::<Result<Vec<_>, _>>()?;
    let expected_ids = expectations.keys().cloned().collect::<Vec<_>>();
    if discovered_ids != expected_ids {
        return Err(format!(
            "corpus/expectation case IDs differ: corpus={discovered_ids:?}, expectations={expected_ids:?}"
        ));
    }

    let output_dir = root.join(OUTPUT_DIR);
    if output_dir.exists() {
        fs::remove_dir_all(&output_dir)
            .map_err(|error| format!("failed to clean {}: {error}", output_dir.display()))?;
    }
    fs::create_dir_all(output_dir.join("work"))
        .map_err(|error| format!("failed to create evidence directory: {error}"))?;

    let rustc_version = rustc_version_verbose()?;
    let rustc_host = rustc_host(&rustc_version)?;
    fs::write(output_dir.join("rustc-version.txt"), &rustc_version)
        .map_err(|error| format!("failed to write rustc identity: {error}"))?;
    fs::write(
        output_dir.join("run-metadata.tsv"),
        format!(
            "field\tvalue\nedition\t2024\nrustc_host\t{rustc_host}\nobservation\tstable_macro_tt_acceptance\n"
        ),
    )
    .map_err(|error| format!("failed to write run metadata: {error}"))?;

    let mut results = Vec::new();
    for path in cases {
        let id = case_id(&path)?;
        let fragment = fs::read_to_string(&path)
            .map_err(|error| format!("failed to read {}: {error}", path.display()))?;
        let expected = expectations
            .get(&id)
            .copied()
            .ok_or_else(|| format!("missing expectation for case `{id}`"))?;

        let case_work = output_dir.join("work").join(&id);
        fs::create_dir_all(&case_work)
            .map_err(|error| format!("failed to create {}: {error}", case_work.display()))?;

        let (ferraxis_acceptance, detail) = observe_ferraxis(&id, &fragment);
        fs::write(case_work.join("ferraxis.txt"), detail)
            .map_err(|error| format!("failed to write Ferraxis evidence for {id}: {error}"))?;

        fs::write(case_work.join("probe.rs"), generate_probe(&fragment))
            .map_err(|error| format!("failed to write rustc probe for {id}: {error}"))?;

        let rustc_output = Command::new("rustc")
            .arg("--edition=2024")
            .arg("--crate-name")
            .arg(format!("ferraxis_probe_{}", id.replace('-', "_")))
            .arg("--emit=metadata")
            .arg("probe.rs")
            .arg("-o")
            .arg("probe.rmeta")
            .current_dir(&case_work)
            .output()
            .map_err(|error| format!("failed to execute rustc for {id}: {error}"))?;

        fs::write(case_work.join("rustc-stdout.txt"), &rustc_output.stdout)
            .map_err(|error| format!("failed to write rustc stdout for {id}: {error}"))?;
        fs::write(case_work.join("rustc-stderr.txt"), &rustc_output.stderr)
            .map_err(|error| format!("failed to write rustc stderr for {id}: {error}"))?;

        let rustc_acceptance = if rustc_output.status.success() {
            Acceptance::Accept
        } else {
            Acceptance::Reject
        };
        let observed = classify(ferraxis_acceptance, rustc_acceptance);
        let rustc_exit = rustc_output
            .status
            .code()
            .map_or_else(|| "signal".to_owned(), |code| code.to_string());

        results.push(CaseResult {
            id,
            expected,
            observed,
            ferraxis: ferraxis_acceptance,
            rustc: rustc_acceptance,
            rustc_exit,
        });
    }

    write_results(&output_dir, &results, &rustc_version, &rustc_host)?;

    let mismatches = results
        .iter()
        .filter(|result| result.expected != result.observed)
        .count();

    println!(
        "ferraxis-diff: {} cases, {} expectation mismatches",
        results.len(),
        mismatches
    );
    println!("ferraxis-diff: evidence {}", output_dir.display());

    if mismatches == 0 {
        Ok(())
    } else {
        Err(format!(
            "{mismatches} differential classification(s) differed from committed expectations"
        ))
    }
}

fn workspace_root() -> Result<PathBuf, String> {
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    manifest
        .parent()
        .and_then(Path::parent)
        .map(Path::to_path_buf)
        .ok_or_else(|| "could not determine workspace root".to_owned())
}

fn discover_cases(dir: &Path) -> Result<Vec<PathBuf>, String> {
    let mut cases = fs::read_dir(dir)
        .map_err(|error| format!("failed to read {}: {error}", dir.display()))?
        .map(|entry| entry.map(|value| value.path()))
        .collect::<Result<Vec<_>, _>>()
        .map_err(|error| format!("failed to read corpus entry: {error}"))?;

    cases.retain(|path| path.extension().and_then(|value| value.to_str()) == Some("rsfrag"));
    cases.sort_by(|left, right| left.file_stem().cmp(&right.file_stem()));

    for path in &cases {
        validate_case_id(&case_id(path)?)?;
    }

    Ok(cases)
}

fn case_id(path: &Path) -> Result<String, String> {
    path.file_stem()
        .and_then(|value| value.to_str())
        .map(str::to_owned)
        .ok_or_else(|| format!("invalid UTF-8 case filename: {}", path.display()))
}

fn validate_case_id(id: &str) -> Result<(), String> {
    let valid = !id.is_empty()
        && id.bytes().all(|byte| {
            byte.is_ascii_lowercase() || byte.is_ascii_digit() || matches!(byte, b'-' | b'_')
        });

    if valid {
        Ok(())
    } else {
        Err(format!("invalid differential case ID `{id}`"))
    }
}

fn read_expectations(path: &Path) -> Result<BTreeMap<String, Classification>, String> {
    let text = fs::read_to_string(path)
        .map_err(|error| format!("failed to read {}: {error}", path.display()))?;
    let mut expectations = BTreeMap::new();

    for (index, line) in text.lines().enumerate() {
        let line_number = index + 1;
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        let mut fields = line.split('\t');
        let id = fields
            .next()
            .ok_or_else(|| format!("missing case ID at {}:{line_number}", path.display()))?;
        let classification = fields.next().ok_or_else(|| {
            format!("missing classification at {}:{line_number}", path.display())
        })?;
        if fields.next().is_some() {
            return Err(format!("too many fields at {}:{line_number}", path.display()));
        }

        validate_case_id(id)?;
        let classification = Classification::parse(classification)?;
        if expectations.insert(id.to_owned(), classification).is_some() {
            return Err(format!("duplicate expectation for case `{id}`"));
        }
    }

    Ok(expectations)
}

fn observe_ferraxis(id: &str, fragment: &str) -> (Acceptance, String) {
    let source = SourceFile::new(format!("{id}.rsfrag"), fragment);
    match lex(&source) {
        Ok(tokens) => (
            Acceptance::Accept,
            format!("accept\ntoken_count\t{}\n", tokens.len()),
        ),
        Err(error) => (
            Acceptance::Reject,
            format!(
                "reject\nkind\t{:?}\nspan\t{}..{}\n",
                error.kind,
                error.span.lo().get(),
                error.span.hi().get()
            ),
        ),
    }
}

fn generate_probe(fragment: &str) -> String {
    let mut probe = String::from(
        "macro_rules! __ferraxis_probe {\n    ($($_token:tt)*) => {};\n}\n\n__ferraxis_probe! {\n",
    );
    probe.push_str(fragment);
    if !fragment.ends_with('\n') {
        probe.push('\n');
    }
    probe.push_str("}\n\nfn main() {}\n");
    probe
}

fn rustc_version_verbose() -> Result<String, String> {
    let output = Command::new("rustc")
        .args(["--version", "--verbose"])
        .output()
        .map_err(|error| format!("failed to execute rustc --version --verbose: {error}"))?;

    if !output.status.success() {
        return Err(format!(
            "rustc --version --verbose failed with status {}",
            output.status
        ));
    }

    String::from_utf8(output.stdout)
        .map_err(|error| format!("rustc --version --verbose was not UTF-8: {error}"))
}

fn rustc_host(version: &str) -> Result<String, String> {
    version
        .lines()
        .find_map(|line| line.strip_prefix("host: "))
        .map(str::to_owned)
        .ok_or_else(|| "rustc --version --verbose did not report a host".to_owned())
}

const fn classify(ferraxis: Acceptance, rustc: Acceptance) -> Classification {
    match (ferraxis, rustc) {
        (Acceptance::Accept, Acceptance::Accept) => Classification::AgreeAccept,
        (Acceptance::Reject, Acceptance::Reject) => Classification::AgreeReject,
        (Acceptance::Reject, Acceptance::Accept) => Classification::FerraxisRejects,
        (Acceptance::Accept, Acceptance::Reject) => Classification::RustcRejects,
    }
}

fn write_results(
    output_dir: &Path,
    results: &[CaseResult],
    rustc_version: &str,
    rustc_host: &str,
) -> Result<(), String> {
    let mut tsv = String::from("case\texpected\tobserved\tferraxis\trustc\trustc_exit\n");
    for result in results {
        tsv.push_str(&format!(
            "{}\t{}\t{}\t{}\t{}\t{}\n",
            result.id,
            result.expected.as_str(),
            result.observed.as_str(),
            result.ferraxis.as_str(),
            result.rustc.as_str(),
            result.rustc_exit
        ));
    }
    fs::write(output_dir.join("results.tsv"), tsv)
        .map_err(|error| format!("failed to write machine-readable evidence: {error}"))?;

    let mut report = String::from(
        "# Ferraxis P0-M018 Differential Lexer Observation\n\n\
This report compares Ferraxis lexer acceptance with a stable rustc macro token-tree acceptance \
probe. It is not a raw rustc lexer-token comparison.\n\n",
    );
    report.push_str("## Run metadata\n\n");
    report.push_str("- Edition: `2024`\n");
    report.push_str(&format!("- rustc host: `{rustc_host}`\n"));
    report.push_str("- Observation: `stable_macro_tt_acceptance`\n\n");
    report.push_str("### rustc identity\n\n```text\n");
    report.push_str(rustc_version.trim_end());
    report.push_str("\n```\n\n## Results\n\n");
    report.push_str("| Case | Expected | Observed | Ferraxis | rustc | Exit |\n");
    report.push_str("| --- | --- | --- | --- | --- | ---: |\n");

    for result in results {
        report.push_str(&format!(
            "| {} | `{}` | `{}` | {} | {} | {} |\n",
            result.id,
            result.expected.as_str(),
            result.observed.as_str(),
            result.ferraxis.as_str(),
            result.rustc.as_str(),
            result.rustc_exit
        ));
    }

    fs::write(output_dir.join("report.md"), report)
        .map_err(|error| format!("failed to write human-readable evidence: {error}"))
}

#[cfg(test)]
mod tests {
    use super::{Acceptance, Classification, classify, generate_probe};

    #[test]
    fn classification_matrix_is_complete() {
        assert_eq!(
            classify(Acceptance::Accept, Acceptance::Accept),
            Classification::AgreeAccept
        );
        assert_eq!(
            classify(Acceptance::Reject, Acceptance::Reject),
            Classification::AgreeReject
        );
        assert_eq!(
            classify(Acceptance::Reject, Acceptance::Accept),
            Classification::FerraxisRejects
        );
        assert_eq!(
            classify(Acceptance::Accept, Acceptance::Reject),
            Classification::RustcRejects
        );
    }

    #[test]
    fn probe_keeps_line_comment_inside_fragment_boundary() {
        let probe = generate_probe("// comment");
        assert!(probe.contains("__ferraxis_probe! {\n// comment\n}\n"));
        assert!(probe.ends_with("fn main() {}\n"));
    }
}
