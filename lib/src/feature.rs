use scenario::Scenario;

pub struct Feature {
    scenarios: Vec<Scenario>,
}

impl Feature {
    pub fn add_scenario(&mut self, scenario: Scenario) {
        self.scenarios.push(scenario);
    }
}
