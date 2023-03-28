use bevy::prelude::Color;
use std::ops::RangeInclusive;
use crate::{helper, neighbours::Neighbourhood};


#[derive(Clone, Copy, PartialEq)]
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

    #[allow(dead_code)]
    pub fn in_range(&self, value: u8) -> bool {
        // self.0[value as usize]
        // *self.0.get(value as usize).unwrap_or(&false)
        if (value as usize) < self.0.len() {
            *self.0.get(value as usize).unwrap()
        } else {
            false
        }
    }

    // pub fn in_range_incorrect(&self, value: u8) -> bool {
    //     *self.0.get(value as usize).unwrap_or(&false)
    // }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Rule {
    pub states: u8,
    pub neighbourhood: Neighbourhood,
    pub birth: Value,
    pub survival_rule: Value,
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ColorMethod {
    Single,
    State,
    DistToCenter,
    Neighbour,
}

impl ColorMethod {
    pub fn color(
        &self,
        c1: Color,
        c2: Color,
        states: u8,
        state: u8,
        neighbours: u8,
        dist_to_center: f32,
    ) -> Color {
        match self {
            ColorMethod::Single => c1,
            ColorMethod::State => {
                let dt = state as f32 / states as f32;
                helper::state_colour(c1, c2, dt)
            }
            ColorMethod::DistToCenter => helper::state_colour(c1, c2, dist_to_center),
            ColorMethod::Neighbour => {
                let dt = neighbours as f32 / 26f32;
                helper::state_colour(c1, c2, dt)
            }
        }
    }
}
