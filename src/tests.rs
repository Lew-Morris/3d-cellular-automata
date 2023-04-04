#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};
    use std::sync::atomic::AtomicU8;
    use bevy::{
      math::{
          ivec3,
          IVec3,
      },
    };
    use utilities::{idx_to_pos, pos_to_idx};
    use crate::cells::multi_threaded::AtomicCell;
    use crate::utilities;
    use super::*;


    #[test]
    fn test_pos_to_idx() {
        let bounds = 32;

        // Test case 1
        let index = 0;
        assert_eq!(
            ivec3(0, 0, 0),
            idx_to_pos(index, bounds)
        );

        // Test case 2
        let index = 55;
        assert_eq!(
            ivec3(23, 1, 0),
            idx_to_pos(index, bounds)
        );

        // Test case 3
        let index = 32768;
        assert_eq!(
            ivec3(0, 0, 32),
            idx_to_pos(index, bounds)
        );
    }

    #[test]
    fn test_idx_to_pos() {
        // Test case 1
        assert_eq!(
            55,
            pos_to_idx(ivec3(23, 1, 0), 32)
        );

        // Test case 2
        assert_eq!(
            0,
            pos_to_idx(ivec3(0, 0, 0), 32)
        );

        // Test case 3
        assert_eq!(
            33824,
            pos_to_idx(ivec3(32, 32, 32), 32)
        );
    }

    #[test]
    fn test_get_atomic_cell() {
        let values = AtomicCell(Arc::new(Mutex::new(vec![
            AtomicU8::new(10),
            AtomicU8::new(20),
            AtomicU8::new(30),
        ])));

        assert_eq!(10, values.get_value(0));
        assert_eq!(20, values.get_value(1));
        assert_eq!(30, values.get_value(2));
    }

    #[test]
    fn test_atomic_cell_get_value() {
        let length = 10;
        let cell = AtomicCell::new(length);
        let value = cell.get_value(0);
        assert_eq!(0, value);

        cell.write_value(0, 10);
        let value = cell.get_value(0);
        assert_eq!(10, value);
    }

    #[test]
    fn test_atomic_cell_constructor() {
        let length = 10;
        let cell = AtomicCell::new(length);
        let value = cell.get_value(0);
        assert_eq!(0, value);
    }
}