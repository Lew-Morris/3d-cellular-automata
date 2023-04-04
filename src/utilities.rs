use bevy::{
    math::{
        ivec3,
        IVec3,
        Vec4
    },
    prelude::Color,
};
use rand::Rng;

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

pub fn idx_to_pos(index: i32, bounds: i32) -> IVec3 {
    // The index to position conversion was broken

    // Old Code:
    // ivec3(index % bounds,index / bounds & bounds,index / bounds / bounds)

    // (Modified) Source: https://stackoverflow.com/a/11712864
    let rem = index % (bounds * bounds);
    ivec3(rem % bounds, rem / bounds, index / (bounds * bounds))
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
