#[derive(Debug)]
pub struct Temperature {
    pub count: u16,
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
}
