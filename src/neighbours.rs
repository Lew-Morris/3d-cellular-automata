use bevy::math::{IVec3};

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Neighbourhood {
    Moore,
    VonNeumann,
}

impl Neighbourhood {
    pub fn get_neighbourhood_iter(&self) -> &'static [IVec3] {
        match self {
            Neighbourhood::Moore => &MOORE[..],
            Neighbourhood::VonNeumann => {
                println!("{:#?}", VON_NEUMANN)
                &VON_NEUMANN[..]
            }
        }
    }
}

// todo! Store these correctly
// pub static VON_NEUMANN: [IVec3; 6] = [
//     IVec3::from_array(IVec3(1, 0, 0)),
//     IVec3::from_array(IVec3(0, 1, 0)),
//     IVec3::from_array(IVec3(0, 0, 1)),
//     IVec3::from_array(IVec3(-1, 0, 0)),
//     IVec3::from_array(IVec3(0, -1, 0)),
//     IVec3::from_array(IVec3(0, 0, -1)),
// ];
//
// pub static MOORE: [IVec3; 26] = [
//     IVec3::from_array(IVec3(-1, -1, -1)),
//     IVec3::from_array(IVec3(0, -1, -1)),
//     IVec3::from_array(IVec3(1, -1, -1)),
//     IVec3::from_array(IVec3(-1, 0, -1)),
//     IVec3::from_array(IVec3(0, 0, -1)),
//     IVec3::from_array(IVec3(1, 0, -1)),
//     IVec3::from_array(IVec3(-1, 1, -1)),
//     IVec3::from_array(IVec3(0, 1, -1)),
//     IVec3::from_array(IVec3(1, 1, -1)),
//     IVec3::from_array(IVec3(-1, -1, 0)),
//     IVec3::from_array(IVec3(0, -1, 0)),
//     IVec3::from_array(IVec3(1, -1, 0)),
//     IVec3::from_array(IVec3(-1, 0, 0)),
//     IVec3::from_array(IVec3(1, 0, 0)),
//     IVec3::from_array(IVec3(-1, 1, 0)),
//     IVec3::from_array(IVec3(0, 1, 0)),
//     IVec3::from_array(IVec3(1, 1, 0)),
//     IVec3::from_array(IVec3(-1, -1, 1)),
//     IVec3::from_array(IVec3(0, -1, 1)),
//     IVec3::from_array(IVec3(1, -1, 1)),
//     IVec3::from_array(IVec3(-1, 0, 1)),
//     IVec3::from_array(IVec3(0, 0, 1)),
//     IVec3::from_array(IVec3(1, 0, 1)),
//     IVec3::from_array(IVec3(-1, 1, 1)),
//     IVec3::from_array(IVec3(0, 1, 1)),
//     IVec3::from_array(IVec3(1, 1, 1)),
// ];