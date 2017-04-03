pub mod mock {
    use pattern::Pattern;
    use regex::Regex;

    const A_PATTERN: &str = "I have cukes in my belly";

    pub struct PatternMock {}

    impl Pattern for PatternMock {
        fn to_regex(&self) -> Regex {
            Regex::new(A_PATTERN).unwrap()
        }

        fn to_string(&self) -> String {
            A_PATTERN.to_string()
        }
    }

    pub fn step_handler() {}
}
