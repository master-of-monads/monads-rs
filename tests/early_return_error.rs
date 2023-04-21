#[test]
fn early_return_compile_errors() {
    let t = trybuild::TestCases::new();
    t.compile_fail("macro_test_cases/early_return_fail.rs")
}