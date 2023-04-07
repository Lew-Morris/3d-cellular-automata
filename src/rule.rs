use bevy::prelude::Color;
use crate::{
    neighbours::Neighbourhood,
    utilities::{
        state_colour,
    },
};
use std::ops::RangeInclusive;
use ColourMethod::{
    Colour1,
    Colour2,
    State,
    DistToCenter,
    Neighbour,
    Index,
};

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
        return self
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

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ColourMethod {
    Colour1,
    Colour2,
    State,
    DistToCenter,
    Neighbour,
    Index,
}

impl ColourMethod {
    // Set the colour method
    pub fn set_colour(
        &self,
        c1: Color,
        c2: Color,
        current_state: u8,
        total_states: u8,
        neighbours: u8,
        distance_to_centre: f32,
        index: usize,
        total_cells: usize,
    ) -> Color {
        match self {
            Colour1 => c1,
            Colour2 => c2,
            State => {
                let gradient = current_state as f32 / total_states as f32;
                state_colour(
                    c1,
                    c2,
                    gradient
                )
            }
            DistToCenter => {
                state_colour(
                    c1,
                    c2,
                    distance_to_centre
                )
            },
            Neighbour => {
                let gradient = neighbours as f32 / 26f32;
                state_colour(
                    c1,
                    c2,
                    gradient
                )
            }
            Index => {
                let gradient = index as f32 / total_cells as f32;
                state_colour(
                    c1,
                    c2,
                    gradient
                )
            }
        }
    }
}
