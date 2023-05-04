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

    // Generate a new value from a range
    pub fn from_range(indices: RangeInclusive<u8>) -> Self {
        let mut result = Value([false; 27]);
        for index in indices {
            result.0[index as usize] = true;
        }
        result
    }

    // Check if a value is valid, i.e. is true
    pub fn is_valid(&self, value: u8) -> bool {
        if (value as usize) < self.0.len() {
            *self.0.get(value as usize).unwrap()
        } else {
            false
        }
    }

    // Change the state of a value
    pub fn change_value(mut self, index: usize) -> Self {
        self.0[index] = !self.0[index];
        return self;
    }

    // Get a specified value
    pub fn get_value(self, index: usize) -> bool {
        self.0[index]
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Rule {
    pub birth: Value,
    pub survival: Value,
    pub neighbourhood: Neighbourhood,
    pub states: u8,
}

#[cfg(test)]
mod rule {
    use super::*;

    #[test]
    fn test_value_new() {
        let indices = [1, 3, 5];
        let value = Value::new(&indices);
        assert_eq!(value.0[0], false);
        assert_eq!(value.0[1], true);
        assert_eq!(value.0[2], false);
        assert_eq!(value.0[3], true);
        assert_eq!(value.0[4], false);
        assert_eq!(value.0[5], true);
        assert_eq!(value.0[6], false);
        assert_eq!(value.0[7], false);
        assert_eq!(value.0[8], false);
        assert_eq!(value.0[9], false);
        assert_eq!(value.0[10], false);
        assert_eq!(value.0[11], false);
        assert_eq!(value.0[12], false);
        assert_eq!(value.0[13], false);
        assert_eq!(value.0[14], false);
        assert_eq!(value.0[15], false);
        assert_eq!(value.0[16], false);
        assert_eq!(value.0[17], false);
        assert_eq!(value.0[18], false);
        assert_eq!(value.0[19], false);
        assert_eq!(value.0[20], false);
        assert_eq!(value.0[21], false);
        assert_eq!(value.0[22], false);
        assert_eq!(value.0[23], false);
        assert_eq!(value.0[24], false);
        assert_eq!(value.0[25], false);
        assert_eq!(value.0[26], false);
    }

    #[test]
    fn test_value_from_range() {
        let range = 3..=8;
        let value = Value::from_range(range);
        assert_eq!(value.0[0], false);
        assert_eq!(value.0[1], false);
        assert_eq!(value.0[2], false);
        assert_eq!(value.0[3], true);
        assert_eq!(value.0[4], true);
        assert_eq!(value.0[5], true);
        assert_eq!(value.0[6], true);
        assert_eq!(value.0[7], true);
        assert_eq!(value.0[8], true);
        assert_eq!(value.0[9], false);
        assert_eq!(value.0[10], false);
        assert_eq!(value.0[11], false);
        assert_eq!(value.0[12], false);
        assert_eq!(value.0[13], false);
        assert_eq!(value.0[14], false);
        assert_eq!(value.0[15], false);
        assert_eq!(value.0[16], false);
        assert_eq!(value.0[17], false);
        assert_eq!(value.0[18], false);
        assert_eq!(value.0[19], false);
        assert_eq!(value.0[20], false);
        assert_eq!(value.0[21], false);
        assert_eq!(value.0[22], false);
        assert_eq!(value.0[23], false);
        assert_eq!(value.0[24], false);
        assert_eq!(value.0[25], false);
        assert_eq!(value.0[26], false);
    }

    #[test]
    fn test_value_is_valid() {
        let indices = [1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21, 23, 25];
        let value = Value::new(&indices);
        assert_eq!(value.is_valid(1), true);
        assert_eq!(value.is_valid(2), false);
        assert_eq!(value.is_valid(3), true);
        assert_eq!(value.is_valid(4), false);
        assert_eq!(value.is_valid(5), true);
        assert_eq!(value.is_valid(6), false);
        assert_eq!(value.is_valid(7), true);
        assert_eq!(value.is_valid(8), false);
        assert_eq!(value.is_valid(9), true);
        assert_eq!(value.is_valid(10), false);
        assert_eq!(value.is_valid(11), true);
        assert_eq!(value.is_valid(12), false);
        assert_eq!(value.is_valid(13), true);
        assert_eq!(value.is_valid(14), false);
        assert_eq!(value.is_valid(15), true);
        assert_eq!(value.is_valid(16), false);
        assert_eq!(value.is_valid(17), true);
        assert_eq!(value.is_valid(18), false);
        assert_eq!(value.is_valid(19), true);
        assert_eq!(value.is_valid(20), false);
        assert_eq!(value.is_valid(21), true);
        assert_eq!(value.is_valid(22), false);
        assert_eq!(value.is_valid(23), true);
        assert_eq!(value.is_valid(24), false);
        assert_eq!(value.is_valid(25), true);
        assert_eq!(value.is_valid(26), false);
    }

    #[test]
    fn test_value_change_value() {
        let indices = [1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21, 23, 25];
        let value = Value::new(&indices);
        let new_value = value.change_value(0);
        assert_eq!(value.get_value(0), false);
        assert_eq!(new_value.get_value(0), true);
        assert_eq!(value.get_value(1), true);
        assert_eq!(new_value.get_value(1), true);
    }

    #[test]
    fn test_rule_creation() {
        let indices1 = [1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21, 23, 25];
        let indices2 = [0, 2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22, 24];
        let birth = Value::new(&indices1);
        let survival = Value::new(&indices2);
        let neighbourhood = Neighbourhood::Moore;
        let states = 2;
        let rule = Rule { birth, survival, neighbourhood, states };
        assert_eq!(rule.birth, birth);
        assert_eq!(rule.survival, survival);
        assert_eq!(rule.neighbourhood, neighbourhood);
        assert_eq!(rule.states, states);
    }
}