use crate::{neighbours::Neighbourhood};

#[derive(Clone, Copy)]
pub struct Rule {
    pub states: u8,
    pub neighbourhood: Neighbourhood, // todo! Make a list of IVec3
    // pub birth: Value todo! Implement 'Value' struct
}

impl Rule {
    pub fn new(states: u8) -> Rule {
        Rule {
            states,
            neighbourhood: Neighbourhood::VonNeumann,
        }
    }

    pub fn get_states(self) -> u8 {
        self.states
    }

    pub fn get_neighbourhood(self) -> Neighbourhood {
        self.neighbourhood
    }
}
