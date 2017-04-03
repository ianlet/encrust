use context::Context;
use step_definition::StepDefinition;

use std::panic::{self, AssertUnwindSafe};

#[derive(PartialEq, Eq, Debug)]
pub enum StepType {
    Given,
    When,
    Then,
}

#[derive(PartialEq, Eq, Debug)]
pub struct Step {
    pub step_type: StepType,
    pub description: String,
}

impl Step {
    pub fn new<T: ToString>(step_type: StepType, description: T) -> Self {
        Step {
            step_type: step_type,
            description: description.to_string(),
        }
    }

    pub fn run(&self, context: &Context, skip: bool) -> StepResult {
        if skip {
            return StepResult::Skipped;
        }

        if let Some(step_definition) = context.match_step(self) {
            self.handle(step_definition)
        } else {
            StepResult::Undefined
        }
    }

    fn handle(&self, step_definition: &StepDefinition) -> StepResult {
        match panic::catch_unwind(AssertUnwindSafe(|| step_definition.handle(&self.description))) {
            Ok(_) => StepResult::Passed,
            Err(_) => StepResult::Failed,
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
pub enum StepResult {
    Passed,
    Failed,
    Skipped,
    Undefined,
}

#[cfg(test)]
mod test {
    use super::{Step, StepResult};
    use super::StepType::*;

    use context::Context;
    use pattern::{Pattern, StepPattern};
    use step_definition::{StepDefinition, StepHandler};

    use hamcrest::prelude::*;
    use testing::mock::{PatternMock, failing_step_handler, passing_step_handler};

    const SKIP: bool = true;
    const DONT_SKIP: bool = false;

    #[test]
    fn it_returns_a_skipped_step_result_when_running_given_it_should_be_skipped() {
        let pattern = PatternMock {};
        let step = Step::new(Given, pattern.to_string());
        let context = Context::new();

        let step_result = step.run(&context, SKIP);

        assert_that!(step_result, is(equal_to(StepResult::Skipped)));
    }

    #[test]
    fn it_returns_an_undefined_step_result_when_running_given_no_matching_step_definition() {
        let pattern = PatternMock {};
        let step = Step::new(Given, pattern.to_string());
        let context = Context::new();
        let step_result = step.run(&context, DONT_SKIP);

        assert_that!(step_result, is(equal_to(StepResult::Undefined)));
    }

    #[test]
    fn it_returns_a_failed_step_result_when_running_given_step_definition_is_failing() {
        let pattern = PatternMock {};
        let step = Step::new(Given, pattern.to_string());
        let context = prepare_context(pattern, failing_step_handler);

        let step_result = step.run(&context, DONT_SKIP);

        assert_that!(step_result, is(equal_to(StepResult::Failed)));
    }

    #[test]
    fn it_returns_a_passed_step_result_when_running_given_step_definition_is_passing() {
        let pattern = PatternMock {};
        let step = Step::new(Given, pattern.to_string());
        let context = prepare_context(pattern, passing_step_handler);

        let step_result = step.run(&context, DONT_SKIP);

        assert_that!(step_result, is(equal_to(StepResult::Passed)));
    }

    fn prepare_context<P: Pattern>(pattern: P, step_handler: StepHandler) -> Context {
        let step_pattern = StepPattern::new(pattern);
        let mut context = Context::new();
        let step_definition = StepDefinition::new(step_pattern, step_handler);
        context.register(step_definition);
        context
    }
}
