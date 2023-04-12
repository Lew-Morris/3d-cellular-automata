use crate::neighbours::Neighbourhood;
use std::ops::RangeInclusive;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Value([bool; 27]);

impl Value {
    pub fn new(indices: &[u8]) -> Self {
        let mut result = Value([false; 27]);
        for index in indices {
            result.0[*index as usize] = true;
        }
        result
    }

    pub fn from_range(indices: RangeInclusive<u8>) -> Self {
        let mut result = Value([false; 27]);
        for index in indices {
            result.0[index as usize] = true;
        }
        result
    }

    pub fn is_valid(&self, value: u8) -> bool {
        if (value as usize) < self.0.len() {
            *self.0.get(value as usize).unwrap()
        } else {
            false
        }
    }

    pub fn change_value(mut self, index: usize) -> Self {
        self.0[index] = !self.0[index];
        return self;
    }

    pub fn get_value(self, index: usize) -> bool {
        self.0[index]
    }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Rule {
    pub birth: Value,
    pub survival: Value,
    pub neighbourhood: Neighbourhood,
    pub states: u8,
}
