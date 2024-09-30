use eframe::egui::{ self, Context, RichText, Ui };
use crate::{
    utils::configs::{ delete_config, get_configs, load_config, save_config },
    ZyferOverlay,
};

pub fn draw_menu_aimbot(zyfer_overlay: &mut ZyferOverlay, ctx: &Context, _ui: &mut Ui) {
    egui::Window
        ::new(RichText::new("AIMBOT").color(zyfer_overlay.settings.heading_color))
        .resizable(false)
        .show(ctx, |ui| {
            ui.group(|ui| {
                ui.vertical(|ui| {
                    ui.checkbox(&mut zyfer_overlay.settings.enable_aimbot, "Enable aimbot");
                    ui.add(
                        egui::Slider
                            ::new(&mut zyfer_overlay.settings.aimbot_fov, 4.0..=18.0)
                            .show_value(true)
                            .text("Fov")
                    );
                    ui.add(
                        egui::Slider
                            ::new(&mut zyfer_overlay.settings.aimbot_smoothing, 4.0..=18.0)
                            .show_value(true)
                            .text("Smooth")
                    );
                })
            })
        });
}

pub fn draw_menu_esp(zyfer_overlay: &mut ZyferOverlay, ctx: &Context, _ui: &mut Ui) {
    egui::Window
        ::new(RichText::new("ESP").color(zyfer_overlay.settings.heading_color))
        .resizable(false)
        .show(ctx, |ui| {
            ui.group(|ui| {
                ui.vertical(|ui| {
                    ui.checkbox(&mut zyfer_overlay.settings.esp_settings.enable_esp, "Enable esp");

                    ui.group(|ui| {
                        ui.checkbox(
                            &mut zyfer_overlay.settings.esp_settings.enable_boxes,
                            "Enable boxes"
                        );

                        if zyfer_overlay.settings.esp_settings.enable_boxes {
                            ui.add(
                                egui::Slider
                                    ::new(
                                        &mut zyfer_overlay.settings.esp_settings.box_size,
                                        1.0..=10.0
                                    )
                                    .show_value(true)
                                    .text("Box size")
                            );

                            ui.horizontal(|ui| {
                                ui.color_edit_button_srgba(
                                    &mut zyfer_overlay.settings.esp_settings.box_color
                                );
                                ui.label("Box color");
                            });

                            ui.add(
                                egui::Slider
                                    ::new(
                                        &mut zyfer_overlay.settings.esp_settings.box_rounding,
                                        1.0..=10.0
                                    )
                                    .show_value(true)
                                    .text("Box rounding")
                            );

                            ui.checkbox(
                                &mut zyfer_overlay.settings.esp_settings.box_enable_shadow,
                                "Enable box shadow"
                            );
                            if zyfer_overlay.settings.esp_settings.box_enable_shadow {
                                ui.add(
                                    egui::Slider
                                        ::new(
                                            &mut zyfer_overlay.settings.esp_settings.box_shadow_blur,
                                            1.0..=10.0
                                        )
                                        .show_value(true)
                                        .text("Box shadow blur")
                                );

                                ui.horizontal(|ui| {
                                    ui.color_edit_button_srgba(
                                        &mut zyfer_overlay.settings.esp_settings.box_shadow_color
                                    );
                                    ui.label("Box shadow color");
                                });

                                ui.add(
                                    egui::Slider
                                        ::new(
                                            &mut zyfer_overlay.settings.esp_settings.box_shadow_size,
                                            1.0..=10.0
                                        )
                                        .show_value(true)
                                        .text("Box shadow size")
                                );
                            }
                        }
                        ui.separator();
                        ui.checkbox(
                            &mut zyfer_overlay.settings.esp_settings.enable_health,
                            "Enable health"
                        );

                        if zyfer_overlay.settings.esp_settings.enable_health {
                            ui.add(
                                egui::Slider
                                    ::new(
                                        &mut zyfer_overlay.settings.esp_settings.health_font_size,
                                        1.0..=50.0
                                    )
                                    .show_value(true)
                                    .text("Health font size")
                            );

                            ui.horizontal(|ui| {
                                ui.color_edit_button_srgba(
                                    &mut zyfer_overlay.settings.esp_settings.health_font_color
                                );
                                ui.label("Health font color");
                            });

                            ui.checkbox(
                                &mut zyfer_overlay.settings.esp_settings.health_shadow,
                                "Health font shadow"
                            );

                            if zyfer_overlay.settings.esp_settings.health_shadow {
                                ui.horizontal(|ui| {
                                    ui.color_edit_button_srgba(
                                        &mut zyfer_overlay.settings.esp_settings.health_shadow_color
                                    );
                                    ui.label("Health shadow color");
                                });
                            }
                        }

                        ui.separator();
                        ui.checkbox(
                            &mut zyfer_overlay.settings.esp_settings.enable_name,
                            "Enable names"
                        );

                        if zyfer_overlay.settings.esp_settings.enable_name {
                            ui.add(
                                egui::Slider
                                    ::new(
                                        &mut zyfer_overlay.settings.esp_settings.name_font_size,
                                        1.0..=50.0
                                    )
                                    .show_value(true)
                                    .text("Name font size")
                            );

                            ui.horizontal(|ui| {
                                ui.color_edit_button_srgba(
                                    &mut zyfer_overlay.settings.esp_settings.name_font_color
                                );
                                ui.label("Name font color");
                            });

                            ui.checkbox(
                                &mut zyfer_overlay.settings.esp_settings.name_shadow,
                                "Name font shadow"
                            );

                            if zyfer_overlay.settings.esp_settings.name_shadow {
                                ui.horizontal(|ui| {
                                    ui.color_edit_button_srgba(
                                        &mut zyfer_overlay.settings.esp_settings.name_shadow_color
                                    );
                                    ui.label("Name shadow color");
                                });
                            }
                        }
                    });

                    /* 
                    ui.checkbox(&mut zyfer_overlay.settings.esp_settings.enable_name, "Enable names");
                    ui.checkbox(&mut zyfer_overlay.settings.esp_settings.enable_health, "Enable health");
                    ui.checkbox(&mut zyfer_overlay.settings.esp_settings.enable_tracer, "Enable tracer");*/
                })
            })
        });
}

pub fn draw_menu_misc(zyfer_overlay: &mut ZyferOverlay, ctx: &Context, _ui: &mut Ui) {
    egui::Window
        ::new(RichText::new("MISC").color(zyfer_overlay.settings.heading_color))
        .resizable(false)
        .show(ctx, |ui| {
            ui.group(|ui| {
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        ui.color_edit_button_srgba(&mut zyfer_overlay.settings.heading_color);
                        ui.label("Menu heading color");
                    });

                    ui.group(|ui| {
                        ui.label("Configs");
                        for file in &zyfer_overlay.configs {
                            let file = file.to_str().unwrap().to_string().replace(".cfg", "");
                            if ui.button(&file).clicked() {
                                zyfer_overlay.current_config = file;
                            }
                        }

                        egui::Grid
                            ::new("esp_grid 2")
                            .num_columns(2)
                            .show(ui, |ui| {
                                ui.text_edit_singleline(&mut zyfer_overlay.current_config);
                                ui.label("Name");
                                ui.end_row();

                                if ui.button("Save").clicked() {
                                    save_config(
                                        &zyfer_overlay.current_config,
                                        &zyfer_overlay.settings
                                    );
                                    zyfer_overlay.configs = get_configs();
                                }
                                if ui.button("Load").clicked() {
                                    zyfer_overlay.settings = load_config(
                                        &zyfer_overlay.current_config
                                    );
                                }

                                ui.end_row();

                                if ui.button("Delete").clicked() {
                                    delete_config(&zyfer_overlay.current_config);
                                    zyfer_overlay.configs = get_configs();
                                }

                                if ui.button("Refresh").clicked() {
                                    zyfer_overlay.configs = get_configs();
                                }
                                ui.end_row();
                            });
                    });
                })
            })
        });
}
