use pattern::{StepArgument, StepPattern};
use step::Step;

use std::collections::HashMap;

pub type StepHandler = fn(arguments: HashMap<String, StepArgument>);

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct StepDefinition {
    pattern: StepPattern,
    handler: StepHandler,
}

impl StepDefinition {
    pub fn new(pattern: StepPattern, handler: StepHandler) -> Self {
        StepDefinition {
            pattern: pattern,
            handler: handler,
        }
    }

    pub fn is_match(&self, step: &Step) -> bool {
        self.pattern.is_match(&step.description)
    }

    pub fn handle(&self, description: &String) {
        let arguments = self.pattern.captures(description);
        (self.handler)(arguments);
    }
}

#[cfg(test)]
mod test {
    use super::StepDefinition;
    use pattern::{Pattern, StepPattern};
    use step::{Step, StepType};

    use testing::mock::{PatternMock, step_handler};

    const A_NON_MATCHING_DESCRIPTION: &str = "This description does not match";

    #[test]
    fn it_is_matching_a_step_given_the_step_description_is_matching_its_pattern() {
        let pattern = PatternMock {};
        let description = pattern.to_string();
        let step_pattern = StepPattern::new(pattern);
        let step = Step::new(StepType::Given, description);
        let step_definition = StepDefinition::new(step_pattern, step_handler);

        assert!(step_definition.is_match(&step));
    }

    #[test]
    fn it_is_not_matching_a_step_given_the_step_description_is_not_matching_its_pattern() {
        let pattern = PatternMock {};
        let step_pattern = StepPattern::new(pattern);
        let step = Step::new(StepType::Given, A_NON_MATCHING_DESCRIPTION);
        let step_definition = StepDefinition::new(step_pattern, step_handler);

        assert!(!step_definition.is_match(&step));
    }
}
