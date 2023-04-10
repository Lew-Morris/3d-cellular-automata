use bevy::prelude::ResMut;

use bevy_egui::{
    egui,
    egui::{Checkbox, ComboBox, Grid, Slider, Window},
    EguiContexts,
};

use crate::{neighbours::Neighbourhood::*, rule::ColourMethod};

use crate::cells::{sims, Sims};

// todo! For each example, add an image to the button, and arrange in a grid
// todo! Allow the user to save the current simulation as an example
//  - Would be better to convert current examples to this and add them dynamically
pub fn settings(mut current: ResMut<Sims>, mut contexts: EguiContexts) {
    if current.active_sim > current.sims.len() {
        current.set_sim(0);
    }
    let mut bounds = current.bounds;
    let mut active_sim = current.active_sim;

    // Settings GUI
    Window::new("Settings").show(contexts.ctx_mut(), |ui| {
        let previous_bounds = bounds;
        let previous_sim = active_sim;

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

            let cell_count = sim.count();
            ui.label(format!("Cells: {}", cell_count));
            ui.label(format!(
                "Update: {:.2?} per cell",
                update_dt / cell_count.max(1) as u32
            ));
            // todo! add a label to show FPS/ frame times

            // Reset the sim
            if ui.button("Reset").clicked() {
                sim.reset();
            }

            // Spawn noise
            if ui.button("Spawn noise").clicked() {
                sim.spawn_noise(&rule);
            }

            // Bounding size slider
            ui.add(Slider::new(&mut bounds, 32..=128).text("Bounding size"));
            {
                if bounds != previous_bounds {
                    bounds = sim.set_bounds(bounds);
                    sim.spawn_noise(&rule);
                    current.renderer.as_mut().unwrap().set_bounds(bounds);
                }
                current.rule = Some(rule);
            }
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
                        "Colour 1",
                    );
                    ui.selectable_value(
                        &mut current.colour_method,
                        ColourMethod::Colour2,
                        "Colour 2",
                    );
                    ui.selectable_value(&mut current.colour_method, ColourMethod::State, "State");
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
                    ui.selectable_value(&mut current.colour_method, ColourMethod::Index, "Index");
                });

            ui.label("Colours");
            {
                // todo! change this
                sims::colour_picker(ui, &mut current.colour1);
                sims::colour_picker(ui, &mut current.colour2);
            }

            let mut rule = current.rule.take().unwrap();
            let previous_rule = rule.clone();

            // Set neighbour method
            ComboBox::from_label("Neighbour Method")
                .selected_text(format!("{:?}", rule.neighbourhood))
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut rule.neighbourhood, Moore, "Moore");
                    ui.selectable_value(&mut rule.neighbourhood, VonNeumann, "Von Neumann");
                });

            // Number of states slider
            ui.add(Slider::new(&mut rule.states, 1..=50).text("Number of States"));

            // If the slider changes, update the rule, and restart the simulation
            if rule != previous_rule {
                let sim = &mut current.sims[active_sim].1;
                sim.reset();
                sim.spawn_noise(&rule);
            }
            current.rule = Some(rule);

            let spacing = egui::vec2(1.0, 1.0);
            ui.add_space(10.0);

            // Birth and Survival rules
            ui.horizontal(|ui| {
                ui.group(|ui| {
                    ui.vertical(|ui| {
                        ui.label("Select Birth Values");
                        {
                            // Birth value checkboxes
                            Grid::new("birth_grid").spacing(spacing).show(ui, |ui| {
                                // todo! grey out values depending on neighbourhood method
                                // Checkbox for each value
                                for i in 0..=26 {
                                    if ui
                                        .add(Checkbox::new(
                                            &mut rule.birth.get_value(i),
                                            format!("{}", i + 1),
                                        ))
                                        .clicked()
                                    {
                                        rule.birth = rule.birth.change_value(i);
                                        // birth_list.push(i);
                                    }
                                    // Every third element, make a new row
                                    if (i + 1) % 3 == 0 {
                                        ui.end_row()
                                    }
                                }
                                // If the values change, save the rule, and restart the simulation
                                if rule != previous_rule {
                                    let sim = &mut current.sims[active_sim].1;
                                    sim.reset();
                                    sim.spawn_noise(&rule);
                                }
                                current.rule = Some(rule);
                            });
                        }
                    });

                    ui.vertical(|ui| {
                        ui.label("Select Survival Values");
                        {
                            Grid::new("survival_grid").spacing(spacing).show(ui, |ui| {
                                for i in 0..=26 {
                                    // todo! Grey out boxes > 6? (check val) if VN nbhd
                                    // Checkbox for each value
                                    if ui
                                        .add(Checkbox::new(
                                            &mut rule.survival.get_value(i),
                                            format!("{}", i + 1),
                                        ))
                                        .clicked()
                                    {
                                        // Update the value
                                        rule.survival = rule.survival.change_value(i);
                                    };

                                    // Every third element, make a new row
                                    if (i + 1) % 3 == 0 {
                                        ui.end_row()
                                    };
                                }
                                // If the values change, save the rule, and restart the simulation
                                if rule != previous_rule {
                                    let sim = &mut current.sims[active_sim].1;
                                    sim.reset();
                                    sim.spawn_noise(&rule);
                                }
                                // Update the current rule
                                current.rule = Some(rule);
                            });
                        }
                    });
                });
            });
        }

        ui.add_space(24.0);

        ui.label("Examples:");
        {
            for i in 0..current.examples.len() {
                let example = &current.examples[i];
                if ui.button(&example.name).clicked() {
                    current.set_example(i);
                }
            }
        }
        let rule = current.rule.take().unwrap();

        // Update all variables
        current.bounds = bounds;
        current.active_sim = active_sim;
        current.rule = Some(rule);
    });
}
