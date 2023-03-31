#[cfg(test)]
mod tests {
    use bevy::math::ivec3;
    use bevy::prelude::IVec3;
    use crate::utilities;

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    // use super::*;

    // POS TO INDEX
    #[test]
    fn test_pos_to_idx_1() {
        let index: i32 = 55;
        let bounds: i32 = 32;
        assert_eq!(ivec3(23, 1, 0), utilities::idx_to_pos(index, bounds))
    }

    #[test]
    fn test_pos_to_idx_2() {
        let index: i32 = 32768;
        let bounds: i32 = 32;
        assert_eq!(ivec3(0, 0, 32), utilities::idx_to_pos(index, bounds))
    }

    #[test]
    fn test_pos_to_idx_3() {
        let index: i32 = 0;
        let bounds: i32 = 32;
        assert_eq!(ivec3(0, 0, 0), utilities::idx_to_pos(index, bounds))
    }

    // INDEX TO POS
    #[test]
    fn test_idx_to_pos_1() {
        let bounds: i32 = 32;
        let position: IVec3 = ivec3(23, 1, 0);
        assert_eq!(55, utilities::pos_to_idx(position, bounds))
    }

    #[test]
    fn test_idx_to_pos_2() {
        let bounds: i32 = 32;
        let position: IVec3 = ivec3(0, 0, 0);
        assert_eq!(0, utilities::pos_to_idx(position, bounds))
    }

    #[test]
    fn test_idx_to_pos_3() {
        let bounds: i32 = 32;
        let position: IVec3 = ivec3(32, 32, 32);
        assert_eq!(33824, utilities::pos_to_idx(position, bounds))
    }

}