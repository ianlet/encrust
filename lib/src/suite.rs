use feature::Feature;

pub struct Suite {
    features: Vec<Feature>,
}

impl Suite {
    pub fn add_feature(&mut self, feature: Feature) {
        self.features.push(feature);
    }
}
