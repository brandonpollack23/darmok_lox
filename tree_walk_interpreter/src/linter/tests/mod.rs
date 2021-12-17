use crate::linter::lint_tokens;
use crate::scanner::scan_with_whitespace;
use crate::LinterError;

#[test]
fn double_space() {
    let results = scan_with_whitespace("print  \"test\"", false);
    let lint_results = lint_tokens(&results);
    assert_eq!(
        lint_results,
        vec![LinterError::DoubleSpaceDetected(1, 6).into()]
    )
}
