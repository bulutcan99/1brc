#[derive(Debug)]
pub struct Temperature {
    pub count: u64,
    pub sum: f64,
    pub min: f64,
    pub max: f64,
}

impl Default for Temperature {
    fn default() -> Self {
        Temperature {
            count: 0,
            sum: 0.0,
            min: f64::MAX,
            max: f64::MIN,
        }
    }
}

impl Temperature {
    pub fn new(temp: f64) -> Self {
        Temperature {
            count: 1,
            sum: temp,
            min: temp,
            max: temp,
        }
    }

    pub fn update(&mut self, temp: f64) {
        self.count += 1;
        self.sum += temp;
        self.min = self.min.min(temp);
        self.max = self.max.max(temp);
    }

    pub fn average(&self) -> f64 {
        self.sum / self.count as f64
    }

    pub fn merge(&mut self, other: &Temperature) {
        self.count += other.count;
        self.sum += other.sum;
        self.min = self.min.min(other.min);
        self.max = self.max.max(other.max);
    }
}
