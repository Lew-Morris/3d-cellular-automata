use bevy::{
    prelude::{
        Color,
        Plugin,
        Query,
        ResMut,
        Resource,
    },
    tasks::{
        AsyncComputeTaskPool,
    },
};

use bevy_egui::{
    egui,
    EguiContexts,
    egui::{
        color_picker,
        Checkbox,
        ComboBox,
        Grid,
        Slider,
        Ui,
        Window,
    },
};

use crate::{
    cells::Sim,
    neighbours::Neighbourhood::*,
    render::{
        CellRenderer,
        InstanceData,
        InstanceMaterialData,
    },
    rule::{
        ColourMethod,
        Rule,
    },
    utilities,
};

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
    sims: Vec<(String, Box<dyn Sim>)>,
    active_sim: usize,
    bounds: i32,
    update_duration: std::time::Duration,
    renderer: Option<Box<CellRenderer>>,
    rule: Option<Rule>,
    colour_method: ColourMethod,
    colour1: Color,
    colour2: Color,
    // paused: bool,

    examples: Vec<Example>,
}

impl Sims {
    pub fn new() -> Sims {
        Sims {
            sims: vec![],
            active_sim: usize::MAX,
            bounds: 100,
            update_duration: std::time::Duration::from_secs(0),
            renderer: Some(Box::new(CellRenderer::new())),
            rule: None,
            colour_method: ColourMethod::DistToCenter,
            colour1: Color::NONE,
            colour2: Color::NONE,
            examples: vec![],
            // paused: true,
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

pub fn update(
    mut current: ResMut<Sims>,
    mut query: Query<&mut InstanceMaterialData>,
    mut contexts: EguiContexts,
) {
    if current.active_sim > current.sims.len() {
        current.set_sim(0);
    }
    let mut bounds = current.bounds;
    let mut active_sim = current.active_sim;

    // Settings GUI
    Window::new("Settings").show(contexts.ctx_mut(), |ui| {
        let old_bounds = bounds;
        let previous_sim = active_sim;

        // todo! clean up the code, formatting etc

        // todo! Add tooltips to each element in the UI
        // ui.hyperlink("https://docs.rs/egui/").on_hover_text("This is a tooltip!");
        ui.label("Simulator:");
        {
            ComboBox::from_id_source("simulator")
                .selected_text(&current.sims[active_sim].0)
                .show_ui(ui, |ui| {
                    for (i, (name, _)) in current.sims.iter().enumerate() {
                        ui.selectable_value(&mut active_sim, i, name);
                    }
                });

            if active_sim != previous_sim {
                current.set_sim(active_sim);
                bounds = current.bounds;
            }

            let update_dt = current.update_duration;
            let rule = current.rule.take().unwrap();
            let sim = &mut current.sims[active_sim].1;

            let cell_count = sim.get_count();
            ui.label(format!("Cells: {}", cell_count));
            ui.label(format!(
                "Update: {:.2?} per cell",
                update_dt / cell_count.max(1) as u32
            ));
            // todo! add a label to show FPS/ frame times

            if ui.button("Reset").clicked() {
                sim.reset();
            }
            if ui.button("Spawn noise").clicked() {
                sim.spawn_noise(&rule);
            }

            ui.add(Slider::new(&mut bounds, 32..=128).text("Bounding size"));
            if bounds != old_bounds {
                bounds = sim.set_bounds(bounds);
                sim.spawn_noise(&rule);
                current.renderer.as_mut().unwrap().set_bounds(bounds);
            }
            current.rule = Some(rule);
        }

        ui.add_space(20.0);

        ui.label("Rules:");
        {
            ComboBox::from_label("Colouring")
                .selected_text(format!("{:?}", current.colour_method))
                .show_ui(ui, |ui| {
                    ui.selectable_value(
                        &mut current.colour_method,
                        ColourMethod::Colour1,
                        "Colour 1");
                    ui.selectable_value(
                        &mut current.colour_method,
                        ColourMethod::Colour2,
                        "Colour 2");
                    ui.selectable_value(
                        &mut current.colour_method,
                        ColourMethod::State,
                        "State",
                    );
                    ui.selectable_value(
                        &mut current.colour_method,
                        ColourMethod::DistToCenter,
                        "Distance to Center",
                    );
                    ui.selectable_value(
                        &mut current.colour_method,
                        ColourMethod::Neighbour,
                        "Neighbors",
                    );
                    ui.selectable_value(
                        &mut current.colour_method,
                        ColourMethod::Index,
                        "Index",
                    );
                });

            ui.label("Colours");
            colour_picker(ui, &mut current.colour1);
            colour_picker(ui, &mut current.colour2);

            let mut rule = current.rule.take().unwrap();
            let previous_rule = rule.clone();

            ComboBox::from_label("Neighbour Method")
                .selected_text(format!("{:?}", rule.neighbourhood))
                .show_ui(ui, |ui| {
                    ui.selectable_value(
                        &mut rule.neighbourhood,
                        Moore,
                        "Moore"
                    );
                    ui.selectable_value(
                        &mut rule.neighbourhood,
                        VonNeumann,
                        "Von Neumann",
                    );
                });

            ui.add(Slider::new(&mut rule.states, 1..=50).text("Number of States"));

            if rule != previous_rule {
                let sim = &mut current.sims[active_sim].1;
                sim.reset();
                sim.spawn_noise(&rule);
            }
            current.rule = Some(rule);

            ui.add_space(10.0);

            let spacing = egui::vec2(1.0, 1.0);
            ui.horizontal(|ui| {
                ui.group(|ui| {
                    ui.vertical(|ui| {
                        ui.label("Select Birth Values");
                        Grid::new("birth_grid")
                            .spacing(spacing)
                            .show(ui, |ui| {
                                // todo! grey out values depending on neighbourhood method
                                for i in 0..=26 {
                                    if ui.add(
                                        Checkbox::new(
                                            &mut rule.birth.get_value(i),
                                            format!("{}",  i + 1))
                                    ).clicked() {
                                        rule.birth = rule.birth.change_value(i);
                                        // birth_list.push(i);
                                    }
                                    if (i + 1) % 3 == 0 {
                                        ui.end_row()
                                    }
                                }
                                // Save the value to the current rule if changed
                                if rule != previous_rule {
                                    let sim = &mut current.sims[active_sim].1;
                                    sim.reset();
                                    sim.spawn_noise(&rule);
                                }
                                current.rule = Some(rule);
                            });
                    });

                    ui.vertical(|ui| {
                        ui.label("Select Survival Values");
                        Grid::new("survival_grid")
                            .spacing(spacing)
                            .show(ui, |ui| {
                                for i in 0..=26 {
                                    if ui.add(
                                        Checkbox::new(
                                            &mut rule.survival_rule.get_value(i),
                                            format!("{}",  i + 1))
                                    ).clicked() {
                                        rule.survival_rule = rule.survival_rule.change_value(i);
                                        // birth_list.push(i);
                                    };
                                    if (i + 1) % 3 == 0 {
                                        ui.end_row()
                                    };
                                };
                                // Save the value to the current rule
                                // current.rule.unwrap().birth = current_birth;
                                if rule != previous_rule {
                                    let sim = &mut current.sims[active_sim].1;
                                    sim.reset();
                                    sim.spawn_noise(&rule);
                                }
                                current.rule = Some(rule);
                            });
                    });
                });
            });

            if rule != previous_rule {
                let sim = &mut current.sims[active_sim].1;
                sim.reset();
                sim.spawn_noise(&rule);
            }
            // current.rule = Some(rule);
        }

        ui.add_space(24.0);

        ui.label("Examples:");
        for i in 0..current.examples.len() {
            let example = &current.examples[i];
            if ui.button(&example.name).clicked() {
                current.set_example(i);
            }
        };
    });

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
                    .color(
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


pub struct SimsPlugin;
impl Plugin for SimsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .insert_resource(Sims::new())
            .add_system(update);
    }
}

fn colour_picker(ui: &mut Ui, colour: &mut Color) {
    let mut c = [
        (colour.r() * 255.0) as u8,
        (colour.g() * 255.0) as u8,
        (colour.b() * 255.0) as u8,
    ];
    color_picker::color_edit_button_srgb(ui, &mut c);
    colour.set_r(c[0] as f32 / 255.0);
    colour.set_g(c[1] as f32 / 255.0);
    colour.set_b(c[2] as f32 / 255.0);
}