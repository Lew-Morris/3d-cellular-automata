use bevy::diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin};
use bevy::prelude::{ClearColor, Color, Res, ResMut};

use bevy_egui::egui::FontFamily::Proportional;
use bevy_egui::egui::{color_picker, FontId, Style, Ui};
use bevy_egui::{
    egui,
    egui::{Checkbox, ComboBox, Grid, Slider, TextStyle::*, Window},
    EguiContexts,
};

use crate::neighbours::Neighbourhood::*;

use crate::cells::Sims;
use crate::color_method::ColourMethod::*;

// todo! For each example, add an image to the button, and arrange in a grid
// todo! Allow the user to save the current simulation as an example
//  - Would be better to convert current examples to this and add them dynamically
pub fn settings(
    mut current: ResMut<Sims>,
    mut contexts: EguiContexts,
    mut clear_color: ResMut<ClearColor>,
    diagnostics: Res<Diagnostics>,
) {
    if current.active_sim > current.sims.len() {
        current.set_sim(0);
    }
    let mut bounds = current.bounds;
    let mut active_sim = current.active_sim;
    // Settings GUI

    // SidePanel::new(Left, "Settings")
    Window::new("Settings")
        .resizable(false)
        .show(contexts.ctx_mut(), |ui| {
            // Change default styling
            let mut style = (*ui.style_mut()).clone();

            // Save current style
            ui.set_style(ui_style(&mut style));
            ui.set_max_width(285.0);

            // Define previous values
            let previous_bounds = bounds;
            let previous_sim = active_sim;

            // todo! Add tooltips to each element in the UI
            // ui.label("https://docs.rs/egui/").on_hover_text("This is a tooltip!");

            ui.heading(format!("Information:")).on_hover_text("Information about the current simulation");
            {
                ui.group(|ui| {
                    ui.set_width(275.0);
                    ui.vertical(|ui| {
                        let update_dt = current.update_duration;
                        let cell_count = current.sims[active_sim].1.count();
                        // Get the current framerate
                        let fps = diagnostics
                            .get(FrameTimeDiagnosticsPlugin::FPS)
                            .unwrap()
                            .smoothed()
                            .unwrap_or(0.0);

                        ui.label(format!("Cells: {}", cell_count));
                        ui.label(format!(
                            "Update: {:.2?} per cell",
                            update_dt / cell_count.max(1) as u32
                        ));
                        ui.label(format!("Framerate: {:.1?}fps", fps));
                    });

                    ui.add_space(10.0);

                    ui.heading(format!("Controls:")).on_hover_text("Controls for the simulation");
                    {
                        ui.collapsing("Show controls", |ui| {
                            ui.horizontal(|ui| {
                                ui.vertical(|ui| {
                                    ui.label("Lock/ Unlock: ");

                                    ui.end_row();
                                    ui.add_space(2.0);

                                    // Direction Controls
                                    ui.label("Direction: ");

                                    ui.end_row();
                                    ui.add_space(2.0);

                                    ui.label("Up: ");

                                    ui.end_row();
                                    ui.add_space(2.0);

                                    ui.label("Down: ");
                                });
                                ui.vertical(|ui| {
                                    ui.button("ESC").on_hover_text("Press escape to unlock/ lock the camera movement");
                                    ui.end_row();
                                    ui.button("WASD").on_hover_text("Use W to move forward, S to go back, A to go left, and D to go right - how you would do this in most games");
                                    ui.end_row();
                                    ui.button("SHIFT").on_hover_text("Press teh Shift key to move the camera upwards");
                                    ui.end_row();
                                    ui.button("CTRL").on_hover_text("Press the Ctrl (Control) key to move the camera downwards");
                                });
                            });
                        });
                    }
                });
            }
            ui.add_space(10.0);

            ui.heading("Simulator Settings:").on_hover_text("Define the behaviour of the cells");
            {
                ui.group(|ui| {
                    ui.set_width(275.0);

                    ui.vertical(|ui| {
                        ui.horizontal(|ui| {
                            ComboBox::from_id_source("Simulator")
                                .selected_text(&current.sims[active_sim].0)
                                .show_ui(ui, |ui| {
                                    for (i, (name, _)) in current.sims.iter().enumerate() {
                                        ui.selectable_value(&mut active_sim, i, name);
                                    }
                                });
                            ui.add_space(10.0);
                            ui.label("Background: ");
                            {
                                // Adapted from: https://github.com/bevyengine/bevy/blob/main/examples/window/clear_color.rs
                                colour_picker(ui, &mut clear_color);
                            }
                        });

                        if active_sim != previous_sim {
                            current.set_sim(active_sim);
                            bounds = current.bounds;
                        }

                        ui.add_space(10.0);

                        let rule = current.rule.take().unwrap();
                        let sim = &mut current.sims[active_sim].1;

                        ui.add_space(10.0);
                        ui.horizontal(|ui| {
                            // Spawn noise
                            if ui.button("Spawn noise").on_hover_text("Spawn a random amount of cells in the center").clicked() {
                                sim.spawn_noise(&rule);
                            }

                            // Reset the sim
                            if ui.button("Reset").on_hover_text("Kill all the cells, and stop the simulation").clicked() {
                                sim.reset();
                            }
                        });

                        ui.add_space(10.0);

                        // Bounding size slider
                        ui.add(Slider::new(&mut bounds, 32..=255).text("Bounds Size")).on_hover_text("Change the size of the bounding box");
                        {
                            if bounds != previous_bounds {
                                bounds = sim.set_bounds(bounds);
                                sim.spawn_noise(&rule);
                                current.renderer.as_mut().unwrap().set_bounds(bounds);
                            }
                            current.rule = Some(rule);
                        }
                    });
                });
            }
            ui.add_space(15.0);

            // Rules Group
            ui.heading("Rules:");
            {
                ui.group(|ui| {
                    ui.set_width(275.0);
                    ui.vertical(|ui| {
                        ComboBox::from_label("Colour Method")
                            .selected_text(format!("{:?}", current.colour_method))
                            .show_ui(ui, |ui| {
                                ui.selectable_value(
                                    &mut current.colour_method,
                                    Colour1,
                                    "Colour 1",
                                ).on_hover_text("Only use the first colour");
                                ui.selectable_value(
                                    &mut current.colour_method,
                                    Colour2,
                                    "Colour 2",
                                ).on_hover_text("Only use the second colour");
                                ui.selectable_value(&mut current.colour_method, State, "State");
                                ui.selectable_value(
                                    &mut current.colour_method,
                                    DistToCenter,
                                    "Distance to Center",
                                ).on_hover_text("Cells are coloured based on their distance to the centre of the area, the further away, the more of the second colour they are");
                                ui.selectable_value(
                                    &mut current.colour_method,
                                    Neighbour,
                                    "Neighbors",
                                );
                                ui.selectable_value(&mut current.colour_method, Index, "Index").on_hover_text("The cell's position denotes where it should be between colour 1 and 2");
                            });

                        ui.add_space(10.0);

                        ui.label("Cell Colours: ").on_hover_text("Pick the colour of the cells");
                        {
                            colour_picker(ui, &mut current.colour1);
                            colour_picker(ui, &mut current.colour2);
                        }

                        ui.add_space(10.0);

                        let mut rule = current.rule.take().unwrap();
                        let previous_rule = rule.clone();

                        // Set neighbour method
                        ComboBox::from_label("Neighbour Method: ")
                            .selected_text(format!("{:?}", rule.neighbourhood))
                            .show_ui(ui, |ui| {
                                ui.selectable_value(&mut rule.neighbourhood, Moore, "Moore").on_hover_text("Maximum of 26 neighbours");
                                ui.selectable_value(
                                    &mut rule.neighbourhood,
                                    VonNeumann,
                                    "Von Neumann",
                                ).on_hover_text("Maximum of 6 neighbours");
                            });

                        // Number of states slider
                        ui.add(Slider::new(&mut rule.states, 1..=255).text("Number of States")).on_hover_text("Change the number of states for the cells");

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
                        ui.heading("Values: ").on_hover_text("Select the number of neighbours for a cell to be birthed, or to survive");
                        {
                            ui.group(|ui| {
                                ui.set_width(250.0);
                                ui.horizontal(|ui| {
                                    ui.vertical(|ui| {
                                        ui.set_width(120.0);

                                        ui.label("Birth Values: ");
                                        {
                                            // Birth value checkboxes
                                            Grid::new("birth_grid").spacing(spacing).show(
                                                ui,
                                                |ui| {
                                                    // ui.add_enabled(false, Checkbox::new(&mut false, "0")); // maybe turn into tooltip
                                                    // Checkbox for each value
                                                    for i in 1..=26 {
                                                        let mut enabled = true;
                                                        if i > 6
                                                            && current.rule.unwrap().neighbourhood
                                                                == VonNeumann
                                                        {
                                                            enabled = false;
                                                        }
                                                        if ui
                                                            .add_enabled(
                                                                enabled,
                                                                Checkbox::new(
                                                                    &mut rule.birth.get_value(i),
                                                                    format!("{}", i),
                                                                ),
                                                            )
                                                            .on_hover_text("Click to select this value. If it is greyed out, try changing the neighbourhood :)").clicked() {
                                                            rule.birth = rule.birth.change_value(i);
                                                        }
                                                        // Every third element, make a new row
                                                        if i % 3 == 0 {
                                                            ui.end_row()
                                                        }
                                                    }
                                                    // Update the current rule
                                                    current.rule = Some(rule);
                                                },
                                            );
                                        }
                                    });
                                    ui.add_space(5.0);

                                    ui.vertical(|ui| {
                                        ui.set_width(120.0);
                                        ui.label("Survival Values");
                                        {
                                            Grid::new("survival_grid").spacing(spacing).show(
                                                ui,
                                                |ui| {
                                                    // ui.add_enabled(false, Checkbox::new(&mut false, "0")); // maybe turn into tooltip
                                                    for i in 1..=26 {
                                                        let mut enabled = true;
                                                        if i > 6
                                                            && current.rule.unwrap().neighbourhood
                                                                == VonNeumann
                                                        {
                                                            enabled = false;
                                                        }
                                                        // Checkbox for each value
                                                        if ui
                                                            .add_enabled(
                                                                enabled,
                                                                Checkbox::new(
                                                                    &mut rule.survival.get_value(i),
                                                                    format!("{}", i),
                                                                ),
                                                            )
                                                            .on_hover_text("Click to select this value. If it is greyed out, try changing the neighbourhood :)").clicked() {
                                                            // Update the value
                                                            rule.survival =
                                                                rule.survival.change_value(i);
                                                        };

                                                        // Every third element, make a new row
                                                        if i % 3 == 0 {
                                                            ui.end_row()
                                                        };
                                                    }
                                                    // Update the current rule
                                                    current.rule = Some(rule);
                                                },
                                            );
                                        }
                                    });
                                });
                            });
                        }
                    });
                });
            }

            ui.add_space(10.0);

            ui.label("Examples:");
            {
                ui.group(|ui| {
                    ui.set_width(275.0);
                    ui.horizontal(|ui| {
                        Grid::new("examples_grid").show(ui, |ui| {
                            for i in 0..current.examples.len() {
                                let example = &current.examples[i];
                                if ui.add(egui::Button::new(&example.name)).on_hover_text(format!("Change the simulation to {}", &example.name)).clicked() {
                                    current.set_example(i);
                                }
                                if (i + 1) % 2 == 0 {
                                    ui.end_row();
                                }
                            }
                        });
                    });
                });
            }
            // Get current rule
            let rule = current.rule.take().unwrap();

            // Update all variables
            current.bounds = bounds;
            current.active_sim = active_sim;
            current.rule = Some(rule);
        });
}

fn ui_style(mut style: &mut Style) -> Style {
    style.text_styles = [
        (Heading, FontId::new(20.0, Proportional)),
        (Name("Heading2".into()), FontId::new(16.0, Proportional)),
        (Name("Context".into()), FontId::new(16.0, Proportional)),
        (Body, FontId::new(14.0, Proportional)),
        (Monospace, FontId::new(12.0, Proportional)),
        (Button, FontId::new(14.0, Proportional)),
        (Small, FontId::new(8.0, Proportional)),
    ]
    .into();

    // style.wrap = Some(true);
    style.to_owned()
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
