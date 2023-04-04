use bevy::math::{IVec3};

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Neighbourhood {
    VonNeumann,
    Moore,
    // todo! Add custom neighbourhoods, and a tool to make them
}

impl Neighbourhood {
    pub fn get_neighbourhood_iter(&self) -> &'static [IVec3] {
        match self {
            Neighbourhood::VonNeumann => &VON_NEUMANN[..],
            Neighbourhood::Moore => &MOORE[..],
        }
    }
}

// Check VN neighbourhood (Cells touching a face of target cell)
pub static VON_NEUMANN: [IVec3; 6] = [
    IVec3::from_array([1, 0, 0]),
    IVec3::from_array([0, 1, 0]),
    IVec3::from_array([0, 0, 1]),
    IVec3::from_array([-1, 0, 0]),
    IVec3::from_array([0, -1, 0]),
    IVec3::from_array([0, 0, -1]),
];

// Check Moore Neighbourhood (All cells around target cell)
pub static MOORE: [IVec3; 26] = [
    IVec3::from_array([-1, -1, -1]),
    IVec3::from_array([0, -1, -1]),
    IVec3::from_array([1, -1, -1]),
    IVec3::from_array([-1, 0, -1]),
    IVec3::from_array([0, 0, -1]),
    IVec3::from_array([1, 0, -1]),
    IVec3::from_array([-1, 1, -1]),
    IVec3::from_array([0, 1, -1]),
    IVec3::from_array([1, 1, -1]),
    IVec3::from_array([-1, -1, 0]),
    IVec3::from_array([0, -1, 0]),
    IVec3::from_array([1, -1, 0]),
    IVec3::from_array([-1, 0, 0]),
    IVec3::from_array([1, 0, 0]),
    IVec3::from_array([-1, 1, 0]),
    IVec3::from_array([0, 1, 0]),
    IVec3::from_array([1, 1, 0]),
    IVec3::from_array([-1, -1, 1]),
    IVec3::from_array([0, -1, 1]),
    IVec3::from_array([1, -1, 1]),
    IVec3::from_array([-1, 0, 1]),
    IVec3::from_array([0, 0, 1]),
    IVec3::from_array([1, 0, 1]),
    IVec3::from_array([-1, 1, 1]),
    IVec3::from_array([0, 1, 1]),
    IVec3::from_array([1, 1, 1]),
];