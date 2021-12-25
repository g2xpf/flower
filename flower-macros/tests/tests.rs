#[cfg(test)]
mod parse_test {
    #[test]
    fn trybuild() {
        let t = trybuild::TestCases::new();
        t.pass("tests/01-empty-flow.rs");
        t.pass("tests/02-complex-flow.rs");
    }
}
