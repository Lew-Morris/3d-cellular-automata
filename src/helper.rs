use bevy::{
    math::{ivec3, IVec3, Vec4},
    prelude::Color,
};
use rand::Rng;
// use std::ops::RangeInclusive;

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

pub fn generate_noise_default<F: FnMut(IVec3)>(centre: IVec3, f: F) {
    generate_noise(centre, 6, 12 * 12 * 12, f)
}

pub fn idx_to_pos(index: usize, bounds: i32) -> IVec3 {
    ivec3(
        index as i32 % bounds,
        index as i32 / bounds & bounds,
        index as i32 / bounds / bounds,
    )
}

pub fn pos_to_idx(pos: IVec3, bounds: i32) -> usize {
    (pos.x + (pos.y * bounds) + (pos.z * bounds * bounds)) as usize
}

// pub fn get_bound_range(
//     bounds: i32,
// ) -> (
//     RangeInclusive<i32>,
//     RangeInclusive<i32>,
//     RangeInclusive<i32>,
// ) {
//     let range_x = 0..=bounds - 1;
//     let range_y = 0..=bounds - 1;
//     let range_z = 0..=bounds - 1;
//     (range_x, range_y, range_z)
// }

pub fn centre(bounds: i32) -> IVec3 {
    let centre: i32 = bounds / 2;
    ivec3(centre, centre, centre)
}

pub fn wrap(pos: IVec3, bounds: i32) -> IVec3 {
    (pos + bounds) % bounds
}

// pub fn in_bounds(pos: IVec3, bounds: i32) -> bool {
//     pos.x < bounds && pos.y < bounds && pos.z < bounds
// }

pub fn dist_to_centre(cell_pos: IVec3, bounds: i32) -> f32 {
    let cell_pos = cell_pos - centre(bounds);
    let max = bounds as f32 / 2.0;
    cell_pos.as_vec3().length() / max
}

pub fn state_colour(color_1: Color, color_2: Color, dt: f32) -> Color {
    let color_1: Vec4 = color_1.into();
    let color_2: Vec4 = color_2.into();
    let dt = dt.clamp(0.0, 1.0);
    ((1.0 - dt) * color_1 + dt * color_2).into()
}
