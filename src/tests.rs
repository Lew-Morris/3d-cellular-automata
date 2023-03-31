#[cfg(test)]
mod tests {
    use bevy::math::ivec3;
    use bevy::prelude::IVec3;
    use crate::utilities;

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    // use super::*;

    #[test]
    fn test_pos_to_idx() {
        let index: i32 = 55;
        let bounds: i32 = 32;

        assert_eq!(ivec3(23, 1, 0), utilities::idx_to_pos(index, bounds))
    }

    #[test]
    fn test_idx_to_pos() {
        let bounds: i32 = 32;
        let position: IVec3 = ivec3(23, 1, 0);

        assert_eq!(55, utilities::pos_to_idx(position, bounds))
    }

}