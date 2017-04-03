use step::Step;

pub struct Scenario {
    steps: Vec<Step>,
}

impl Scenario {
    pub fn add_step(&mut self, step: Step) {
        self.steps.push(step);
    }
}
