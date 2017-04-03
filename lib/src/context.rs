use step::Step;
use step_definition::StepDefinition;

pub struct Context {
    step_definitions: Vec<StepDefinition>,
}

impl Context {
    pub fn new() -> Self {
        Context { step_definitions: Vec::new() }
    }

    pub fn register(&mut self, step_definition: StepDefinition) {
        self.step_definitions.push(step_definition);
    }

    pub fn match_step(&self, step: &Step) -> Option<&StepDefinition> {
        self.step_definitions
            .iter()
            .filter(|step_definition| step_definition.is_match(step))
            .next()
    }
}

#[cfg(test)]
mod test {
    use super::Context;

    use pattern::{Pattern, StepPattern};
    use step::{Step, StepType};
    use step_definition::StepDefinition;

    use hamcrest::prelude::*;
    use testing::mock::{PatternMock, step_handler};

    const A_NON_MATCHING_DESCRIPTION: &str = "it does not match";

    #[test]
    fn it_registers_a_step_definition() {
        let mut context = Context::new();
        let pattern = PatternMock {};
        let description = pattern.to_string();
        let step_pattern = StepPattern::new(pattern);
        let step = Step::new(StepType::Given, description);
        let step_definition = StepDefinition::new(step_pattern, step_handler);

        context.register(step_definition);

        assert!(context.match_step(&step).is_some());
    }

    #[test]
    fn it_matches_a_step_with_a_step_definition_given_step_is_matching_a_registered_step_definition() {
        let mut context = Context::new();
        let pattern = PatternMock {};
        let description = pattern.to_string();
        let step_pattern = StepPattern::new(pattern);
        let step = Step::new(StepType::Given, description);
        let step_definition = StepDefinition::new(step_pattern, step_handler);
        context.register(step_definition.clone());

        let matched_step_definition = context.match_step(&step).unwrap();

        assert_that!(matched_step_definition, is(equal_to(&step_definition)));
    }

    #[test]
    fn it_does_not_match_a_step_with_a_step_definition_given_step_is_not_matching_any_registered_step_definition() {
        let mut context = Context::new();
        let pattern = PatternMock {};
        let step_pattern = StepPattern::new(pattern);
        let step = Step::new(StepType::Given, A_NON_MATCHING_DESCRIPTION);
        let step_definition = StepDefinition::new(step_pattern, step_handler);
        context.register(step_definition.clone());

        let matched_step_definition = context.match_step(&step);

        assert!(matched_step_definition.is_none());
    }
}
