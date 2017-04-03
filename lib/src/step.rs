pub enum StepType {
    Given,
    When,
    Then,
}

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
}
