#[derive(Debug)]
pub struct Temperature {
    pub count: u64,
    pub sum: f32,
    pub min: f32,
    pub max: f32,
}

impl Default for Temperature {
    fn default() -> Self {
        Self {
            min: f32::MAX,
            max: f32::MIN,
            count: 0,
            sum: 0.0,
        }
    }
}

impl Temperature {
    pub fn add(&mut self, value: f32) {
        self.count += 1;
        self.sum += value;
        self.min = self.min.min(value);
        self.max = self.max.max(value);
    }

    pub fn average(&self) -> f32 {
        self.sum / self.count as f32
    }

    pub fn merge(&mut self, other: &Temperature) {
        self.count += other.count;
        self.sum += other.sum;
        self.min = self.min.min(other.min);
        self.max = self.max.max(other.max);
    }
}
