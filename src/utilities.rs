use bevy::{
    math::{ivec3, IVec3, Vec4},
    prelude::Color,
};
use rand::Rng;

pub fn idx_to_pos(index: i32, bounds: i32) -> IVec3 {
    // The index to position conversion was broken

    // Old Code:
    // ivec3(index % bounds,index / bounds & bounds,index / bounds / bounds)

    // (Modified) Source: https://stackoverflow.com/a/11712864
    let rem = index % (bounds * bounds);
    ivec3(rem % bounds, rem / bounds, index / (bounds * bounds))
}

pub fn generate_noise<F: FnMut(IVec3)>(centre: IVec3, radius: i32, amount: usize, mut f: F) {
    let mut rand = rand::thread_rng();
    (0..amount).for_each(|_| {
        f(centre
            + ivec3(
            rand.gen_range(-radius..=radius),
            rand.gen_range(-radius..=radius),
            rand.gen_range(-radius..=radius),
        ));
    });
}

pub fn default_noise<F: FnMut(IVec3)>(centre: IVec3, f: F) {
    generate_noise(centre, 10, 8 * 8 * 8, f)
}

pub fn pos_to_idx(position: IVec3, bounds: i32) -> usize {
    (position.x + (position.y * bounds) + (position.z * bounds * bounds)) as usize
}

pub fn get_centre(bounds: i32) -> IVec3 {
    let centre: i32 = bounds / 2;
    ivec3(centre, centre, centre)
}

pub fn wrap(position: IVec3, bounds: i32) -> IVec3 {
    (position + bounds) % bounds
}

pub fn get_dist_to_centre(position: IVec3, bounds: i32) -> f32 {
    let pos = position - get_centre(bounds);
    let max = bounds as f32 / 2.0;
    pos.as_vec3().length() / max
}

pub fn state_colour(colour1: Color, colour2: Color, gradient: f32) -> Color {
    let c1: Vec4 = colour1.into();
    let c2: Vec4 = colour2.into();
    let grad = gradient.clamp(0.0, 1.0);
    ((1.0 - grad) * c1 + grad * c2).into()
}

// TESTS
#[cfg(test)]
mod utils {
    use approx::assert_relative_eq;
    use std::collections::HashSet;
    use super::*;

    #[test]
    fn test_pos_to_idx() {
        let bounds = 32;

        // Test case 1
        let index = 0;
        assert_eq!(ivec3(0, 0, 0), idx_to_pos(index, bounds));

        // Test case 2
        let index = 55;
        assert_eq!(ivec3(23, 1, 0), idx_to_pos(index, bounds));

        // Test case 3
        let index = 32768;
        assert_eq!(ivec3(0, 0, 32), idx_to_pos(index, bounds));
    }

    #[test]
    fn test_idx_to_pos() {
        // Test case 1
        assert_eq!(55, pos_to_idx(ivec3(23, 1, 0), 32));

        // Test case 2
        assert_eq!(0, pos_to_idx(ivec3(0, 0, 0), 32));

        // Test case 3
        assert_eq!(33824, pos_to_idx(ivec3(32, 32, 32), 32));
    }

    // Test generate_noise function
    #[test]
    fn test_generate_noise() {
        let mut result_set = HashSet::new();
        generate_noise(ivec3(0, 0, 0), 1, 10, |p| {
            result_set.insert(p);
        });
        assert!(result_set.len() > 0);
    }

    // Test get_centre function
    #[test]
    fn test_get_centre() {
        assert_eq!(get_centre(3), ivec3(1, 1, 1));
        assert_eq!(get_centre(4), ivec3(2, 2, 2));
    }

    // Test wrap function
    #[test]
    fn test_wrap() {
        assert_eq!(wrap(ivec3(-1, -1, -1), 3), ivec3(2, 2, 2));
        assert_eq!(wrap(ivec3(3, 3, 3), 3), ivec3(0, 0, 0));
    }

    // Test get_dist_to_centre function
    #[test]
    fn test_get_dist_to_centre() {
        let pos = ivec3(1, 1, 1);
        let bounds = 4;
        let dist = get_dist_to_centre(pos, bounds);
        assert_relative_eq!(dist, 0.8660254, epsilon = 1e-6);
    }

    // Test state_colour function
    #[test]
    fn test_state_colour() {
        let c1 = Color::rgb(1.0, 0.0, 0.0);
        let c2 = Color::rgb(0.0, 1.0, 0.0);
        let grad = 0.5;
        let result = state_colour(c1, c2, grad);
        assert_relative_eq!(result.r(), 0.5, epsilon = 1e-6);
        assert_relative_eq!(result.g(), 0.5, epsilon = 1e-6);
        assert_relative_eq!(result.b(), 0.0, epsilon = 1e-6);
    }
}
