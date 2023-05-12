// Adapted from TanTanDev
use bevy::{
    prelude::{Color, Plugin, Query, ResMut, Resource, IntoSystemConfig},
    tasks::AsyncComputeTaskPool,
};


use crate::{
    cells::Sim,
    render::{CellRenderer, InstanceData, InstanceMaterialData},
    rule::Rule,
    utilities,
};

use crate::cells::settings::*;
use crate::color_method::ColourMethod;

#[derive(Clone)]
pub struct Example {
    pub name: String,
    pub rule: Rule,
    pub colour_method: ColourMethod,
    pub colour1: Color,
    pub colour2: Color,
}

#[derive(Resource)]
pub struct Sims {
    pub sims: Vec<(String, Box<dyn Sim>)>,
    pub active_sim: usize,
    pub bounds: i32,
    pub update_duration: std::time::Duration,
    pub renderer: Option<Box<CellRenderer>>,
    pub rule: Option<Rule>,
    pub colour_method: ColourMethod,
    pub colour1: Color,
    pub colour2: Color,
    pub examples: Vec<Example>,
}

impl Sims {
    pub fn new() -> Sims {
        Sims {
            sims: vec![],
            active_sim: usize::MAX,
            bounds: 50,
            update_duration: std::time::Duration::from_secs(0),
            renderer: Some(Box::new(CellRenderer::new())),
            rule: None,
            colour_method: ColourMethod::DistToCenter,
            colour1: Color::NONE,
            colour2: Color::NONE,
            examples: vec![],
        }
    }

    pub fn add_sim(&mut self, name: String, sim: Box<dyn Sim>) {
        self.sims.push((name, sim));
    }

    pub fn add_example(&mut self, example: Example) {
        self.examples.push(example);
    }

    pub fn set_sim(&mut self, index: usize) {
        if self.active_sim < self.sims.len() {
            self.sims[self.active_sim].1.reset();
        }

        let rule: Rule = self.rule.take().unwrap();

        self.active_sim = index;
        self.bounds = self.sims[index].1.set_bounds(self.bounds);
        self.sims[index].1.spawn_noise(&rule);
        self.renderer.as_mut().unwrap().set_bounds(self.bounds);
        self.rule = Some(rule);
    }

    pub fn set_example(&mut self, index: usize) {
        let example = self.examples[index].clone();
        let rule = example.rule;
        self.colour_method = example.colour_method;
        self.colour1 = example.colour1;
        self.colour2 = example.colour2;

        if self.active_sim < self.sims.len() {
            let sim = &mut self.sims[self.active_sim].1;
            sim.reset();
            sim.spawn_noise(&rule);
        }
        self.rule = Some(rule);
    }
}

pub struct SimsPlugin;
impl Plugin for SimsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(Sims::new())
            .add_system(settings_ui.before(update))
            .add_system(update);
    }
}

pub fn update(
    mut current: ResMut<Sims>,
    mut query: Query<&mut InstanceMaterialData>,
) {
    if current.active_sim > current.sims.len() {
        current.set_sim(0);
    }
    let bounds = current.bounds;
    let active_sim = current.active_sim;
    let rule = current.rule.take().unwrap();
    let mut renderer = current.renderer.take().unwrap();

    let sim = &mut current.sims[active_sim].1;

    let t0 = std::time::Instant::now();
    sim.update(&rule, AsyncComputeTaskPool::get());
    let update_dt = t0.elapsed();
    sim.render(&mut renderer);

    let instance_data = &mut query.iter_mut().next().unwrap().0;
    instance_data.truncate(0);
    for index in 0..renderer.cell_count() {
        let value = renderer.values[index];
        let neighbors = renderer.neighbors[index];

        if value != 0 {
            let pos = utilities::idx_to_pos(index as i32, bounds);
            instance_data.push(InstanceData {
                position: (pos - utilities::get_centre(bounds)).as_vec3(),
                scale: 1.0,
                color: current
                    .colour_method
                    .set_colour(
                        current.colour1,
                        current.colour2,
                        value,
                        rule.states,
                        neighbors,
                        utilities::get_dist_to_centre(pos, bounds),
                        index,
                        renderer.cell_count(),
                    )
                    .into(),
            });
        }
    }
    current.bounds = bounds;
    current.active_sim = active_sim;
    current.update_duration = update_dt;
    current.renderer = Some(renderer);
    current.rule = Some(rule);
}

#[cfg(test)]
mod tests {
    use crate::cells::{multi_dimensional};
    use crate::neighbours::Neighbourhood::*;
    use crate::rule::Value;
    use super::*;

    #[test]
    fn test_new_sims() {
        let sims = Sims::new();
        assert_eq!(sims.sims.len(), 0);
        assert_eq!(sims.active_sim, usize::MAX);
        assert_eq!(sims.bounds, 50);
        assert_eq!(sims.update_duration.as_nanos(), 0);
        assert!(sims.renderer.is_some());
        assert!(sims.rule.is_none());
        assert_eq!(sims.colour_method, ColourMethod::DistToCenter);
        assert_eq!(sims.colour1, Color::NONE);
        assert_eq!(sims.colour2, Color::NONE);
        assert_eq!(sims.examples.len(), 0);
    }

    #[test]
    fn test_add_sim() {
        let mut sims = Sims::new();
        let name = String::from("TestSim");
        let sim = Box::new(multi_dimensional::MultiDimensional::new());
        sims.add_sim(name.clone(), sim);
        assert_eq!(sims.sims.len(), 1);
        assert_eq!(sims.sims[0].0, name);
    }

    #[test]
    fn test_add_example() {
        let mut sims = Sims::new();
        let example = Example {
            name: "TestExample".into(),
            rule: Rule {
                survival: Value::new(&[4]),
                birth: Value::new(&[4]),
                states: 5,
                neighbourhood: Moore,
            },
            colour_method: ColourMethod::DistToCenter,
            colour1: Color::RED,
            colour2: Color::BLUE,
        };
        sims.add_example(example);
        assert_eq!(sims.examples.len(), 1);
        assert_eq!(sims.examples[0].name, String::from("TestExample"));
    }

    #[test]
    fn test_set_example() {
        let mut sims = Sims::new();
        let rule = Rule {
            survival: Value::new(&[4]),
            birth: Value::new(&[4]),
            states: 5,
            neighbourhood: Moore,
        };
        let example = Example {
            name: "TestExample".into(),
            rule,
            colour_method: ColourMethod::DistToCenter,
            colour1: Color::RED,
            colour2: Color::BLUE,
        };
        let sim = Box::new(multi_dimensional::MultiDimensional::new());
        sims.add_sim("TestSim".into(), sim);
        sims.add_example(example);
        sims.set_example(0);
        assert_eq!(sims.colour_method, ColourMethod::DistToCenter);
        assert_eq!(sims.colour1, Color::RED);
        assert_eq!(sims.colour2, Color::BLUE);
        assert_eq!(sims.rule.unwrap(), rule);
    }
}