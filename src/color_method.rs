use crate::utilities;
use bevy::prelude::Color;
use utilities::state_colour;
use ColourMethod::{Colour1, Colour2, DistToCenter, Index, Neighbour, State};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ColourMethod {
    Colour1,
    Colour2,
    State,
    DistToCenter,
    Neighbour,
    Index,
}

#[allow(clippy::too_many_arguments)]
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
                state_colour(c1, c2, gradient)
            }
            DistToCenter => state_colour(c1, c2, distance_to_centre),
            Neighbour => {
                let gradient = neighbours as f32 / 26f32;
                state_colour(c1, c2, gradient)
            }
            Index => {
                let gradient = index as f32 / total_cells as f32;
                state_colour(c1, c2, gradient)
            }
        }
    }
}
