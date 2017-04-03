mod turnip_pattern;

use regex::Regex;

pub trait Pattern {
    fn to_regex(&self) -> Regex;
    fn to_string(&self) -> String;
}


#[derive(Clone, Debug)]
pub struct StepPattern {
    source: String,
    regex: Regex,
}

impl StepPattern {
    pub fn new<P: Pattern>(pattern: P) -> Self {
        let regex = pattern.to_regex();
        let source = pattern.to_string();

        StepPattern {
            source: source,
            regex: regex,
        }
    }

    pub fn is_match(&self, description: &String) -> bool {
        self.regex.is_match(description)
    }
}

impl PartialEq for StepPattern {
    fn eq(&self, other: &StepPattern) -> bool {
        self.source == other.source
    }
}

impl Eq for StepPattern {}

#[cfg(test)]
mod test {
    use super::{Pattern, StepPattern};
    use regex::Regex;

    const A_PATTERN: &str = "I have cukes in my belly";
    const A_DESCRIPTION: &str = "It does not match pattern";

    #[test]
    fn it_is_matching_a_description_that_matches_its_pattern() {
        let pattern = PatternMock {};
        let description = pattern.to_string();
        let step_pattern = StepPattern::new(pattern);

        assert!(step_pattern.is_match(&description));
    }

    #[test]
    fn it_is_not_matching_a_description_that_does_not_match_its_pattern() {
        let pattern = PatternMock {};
        let step_pattern = StepPattern::new(pattern);

        assert!(!step_pattern.is_match(&A_DESCRIPTION.to_string()));
    }

    struct PatternMock {}

    impl Pattern for PatternMock {
        fn to_regex(&self) -> Regex {
            Regex::new(A_PATTERN).unwrap()
        }

        fn to_string(&self) -> String {
            A_PATTERN.to_string()
        }
    }
}
