use std::fs;
use std::path::Path;

use pyfmt_python_formatter as pyfmt;
use ruff_python_formatter as ruff;

const CORPUS: &[&str] = &[
    "basic.py",
    "classes.py",
    "collections.py",
    "comments.py",
    "functions.py",
    "pattern_matching.py",
    "strings.py",
    "stubs.pyi",
    "type_params.py",
];

#[test]
fn formats_like_ruff_by_default() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let compat_dir = manifest_dir.join("resources/compat");

    for fixture in CORPUS {
        let path = compat_dir.join(fixture);
        let source = fs::read_to_string(&path)
            .unwrap_or_else(|error| panic!("failed to read {}: {error}", path.display()));

        let ruff_options = ruff::PyFormatOptions::from_extension(&path);
        let pyfmt_options = pyfmt::PyFormatOptions::from_extension(&path);

        let ruff_output = ruff::format_module_source(&source, ruff_options)
            .unwrap_or_else(|error| panic!("ruff failed on {}: {error}", path.display()))
            .into_code();

        let pyfmt_output = pyfmt::format_module_source(&source, pyfmt_options)
            .unwrap_or_else(|error| panic!("pyfmt failed on {}: {error}", path.display()))
            .into_code();

        assert_eq!(pyfmt_output, ruff_output, "fixture: {}", path.display());
    }
}
