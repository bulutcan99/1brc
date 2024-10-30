use std::ops::{Add, Div};

#[derive(Debug, Copy, Clone)]
pub struct Value32(i32);

impl Value32 {
    pub fn parse(mut s: &[u8]) -> Self {
        let neg = if s[0] == b'-' {
            s = &s[1..];
            true
        } else {
            false
        };

        let (a, b, c, d) = match s {
            [c, b'.', d] => (0, 0, c - b'0', d - b'0'),
            [b, c, b'.', d] => (0, b - b'0', c - b'0', d - b'0'),
            [a, b, c, b'.', d] => (a - b'0', b - b'0', c - b'0', d - b'0'),
            [c] => (0, 0, 0, c - b'0'),
            [b, c] => (0, b - b'0', c - b'0', 0),
            [a, b, c] => (a - b'0', b - b'0', c - b'0', 0),
            _ => panic!("Unknown pattern {:?}", std::str::from_utf8(s).unwrap()),
        };

        let v = Value32(a as i32 * 1000 + b as i32 * 100 + c as i32 * 10 + d as i32);

        if neg {
            Self(-v.0)
        } else {
            v
        }
    }

    pub fn format(&self) -> String {
        format!("{:.1}", self.0 as f64 / 10.0)
    }
}

impl Add for Value32 {
    type Output = Value32;

    fn add(self, other: Value32) -> Value32 {
        Value32(self.0 + other.0)
    }
}

impl Div<u64> for Value32 {
    type Output = Value32;

    fn div(self, divisor: u64) -> Value32 {
        Value32(self.0 / divisor as i32)
    }
}

#[derive(Debug)]
pub struct Temperature {
    pub count: u64,
    pub sum: Value32,
    pub min: Value32,
    pub max: Value32,
}

impl Default for Temperature {
    fn default() -> Self {
        Self {
            min: Value32(i32::MAX),
            max: Value32(i32::MIN),
            count: 0,
            sum: Value32(0),
        }
    }
}

impl Temperature {
    pub fn add(&mut self, value: Value32) {
        self.count += 1;
        self.sum = self.sum + value;
        self.min = Value32(self.min.0.min(value.0));
        self.max = Value32(self.max.0.max(value.0));
    }

    pub fn average(&self) -> Value32 {
        if self.count == 0 {
            Value32(0)
        } else {
            self.sum / self.count
        }
    }

    pub fn merge(&mut self, other: &Temperature) {
        self.count += other.count;
        self.sum = self.sum + other.sum;
        self.min = Value32(self.min.0.min(other.min.0));
        self.max = Value32(self.max.0.max(other.max.0));
    }
}
