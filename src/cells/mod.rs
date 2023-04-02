use bevy::{
    tasks::TaskPool
};

use crate::{
    rule::Rule,
    render::CellRenderer
};

pub trait Sim: Send + Sync {
    fn update(&mut self, rule: &Rule, task_pool: &TaskPool);
    fn render(&self, data: &mut CellRenderer);

    fn reset(&mut self) {
        let bounds = self.get_bounds();
        self.set_bounds(0);
        self.set_bounds(bounds);
    }

    fn spawn_noise(&mut self, rule: &Rule);

    fn get_count(&self) -> usize;

    fn get_bounds(&self) -> i32;

    fn set_bounds(&mut self, new_bounds: i32) -> i32;
}

pub mod sims;
pub use sims::*;

pub mod single_threaded;
