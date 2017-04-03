mod turnip_pattern;

use std::collections::HashMap;
use regex::{Captures, CaptureNames, Regex};

pub trait Pattern {
    fn to_regex(&self) -> Regex;
    fn to_string(&self) -> String;
}

#[derive(PartialEq, Eq, Debug)]
pub enum StepArgument {
    String(String),
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

    pub fn captures(&self, description: &String) -> HashMap<String, StepArgument> {
        match self.regex.captures(description) {
            Some(arguments) => extract_step_arguments(self.regex.capture_names(), arguments),
            None => HashMap::new(),
        }
    }
}

fn extract_step_arguments(argument_names: CaptureNames, arguments: Captures) -> HashMap<String, StepArgument> {
    let mut step_arguments = HashMap::new();
    for argument_name in argument_names {
        match argument_name {
            Some(name) => {
                let argument = arguments[name].to_string();
                let step_argument = StepArgument::String(argument);
                step_arguments.insert(name.to_string(), step_argument);
            }
            None => (),
        }
    }
    step_arguments
}

impl PartialEq for StepPattern {
    fn eq(&self, other: &StepPattern) -> bool {
        self.source == other.source
    }
}

impl Eq for StepPattern {}

#[cfg(test)]
mod test {
    use super::{Pattern, StepPattern, StepArgument};

    use pattern::turnip_pattern::TurnipPattern;

    use hamcrest::prelude::*;
    use testing::mock::PatternMock;

    const A_PATTERN: &str = "I have cukes in my belly";
    const A_PATTERN_WITH_ARGUMENTS: &str = "I eat :cuke_count cukes";

    const A_DESCRIPTION: &str = "It does not match pattern";
    const A_DESCRIPTION_WITH_ARGUMENTS: &str = "I eat 7 cukes";

    const CUKE_COUNT: &str = "7";

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

    #[test]
    fn it_captures_step_arguments_from_a_description_given_a_pattern_with_arguments() {
        let pattern = TurnipPattern::from(A_PATTERN_WITH_ARGUMENTS);
        let description = A_DESCRIPTION_WITH_ARGUMENTS.to_string();
        let step_pattern = StepPattern::new(pattern);

        let step_arguments = step_pattern.captures(&description);

        let ref step_argument = step_arguments["cuke_count"];
        let expected_step_argument = StepArgument::String(CUKE_COUNT.to_string());
        assert_that!(step_argument, is(equal_to(&expected_step_argument)));
    }

    #[test]
    fn it_does_not_capture_step_arguments_from_a_description_given_a_pattern_without_arguments() {
        let pattern = TurnipPattern::from(A_PATTERN);
        let description = A_DESCRIPTION.to_string();
        let step_pattern = StepPattern::new(pattern);

        let step_arguments = step_pattern.captures(&description);

        assert!(step_arguments.is_empty());
    }
}
