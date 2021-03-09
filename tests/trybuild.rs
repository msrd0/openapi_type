use trybuild::TestCases;

#[test]
fn trybuild() {
	let t = TestCases::new();
	t.compile_fail("tests/fail/*.rs");
}
