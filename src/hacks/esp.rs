use eframe::{
    egui::{self, Color32, Pos2, Rect, Stroke},
    epaint::RectShape,
};

use crate::{
    offsets::client_dll::cs2_dumper::schemas::client_dll::EntitySpottedState_t,
    settings::settings::Settings,
    utils::{
        math::{world_to_screen, Vector2},
        renderer::{draw_health, draw_name},
    },
};

use super::cheats::Cheats;

pub fn draw_esp(cheats: &Cheats, ui: &mut egui::Ui, settings: &Settings) {
    let g = ui.painter();
    for player in &cheats.players {
        let entity = &player.entity;

        if entity.team == cheats.local_player.team {
            continue;
        }

        if entity.life_state != 256 {
            continue;
        }

        let mut new_entity_pos = entity.position;
        new_entity_pos.z -= 10.0;

        let mut new_view_offset = entity.vec_view_offset;
        new_view_offset.z += 10.0;

        let player_position_2d =
            world_to_screen(&cheats.view_matrix, new_entity_pos, cheats.screen_size);

        //auto new_view_offset = Vector3Sum(pos, view_offset);

        let new_view_offset = entity.position.sum(new_view_offset);

        let view_position_2d =
            world_to_screen(&cheats.view_matrix, new_view_offset, cheats.screen_size);

        let entity_height = player_position_2d.y - view_position_2d.y;

        let rectTop = Vector2 {
            x: view_position_2d.x - entity_height / 3.0,
            y: view_position_2d.y,
        };

        let rectBottom = Vector2 {
            x: player_position_2d.x + entity_height / 3.0,
            y: player_position_2d.y,
        };

        if rectTop.x < 0.0
            || rectTop.y < 0.0
            || rectTop.x > cheats.screen_size.x
            || rectTop.y > cheats.screen_size.y
        {
            continue;
        }

        let rect = Rect {
            min: Pos2 {
                x: rectTop.x,
                y: rectTop.y,
            },
            max: Pos2 {
                x: rectBottom.x,
                y: rectBottom.y,
            },
        };
        let size = settings.esp_settings.box_shadow_size;
        let blur = settings.esp_settings.box_shadow_blur;
        let shadow_color = settings.esp_settings.box_shadow_color;
        let stroke = Stroke::new(
            settings.esp_settings.box_size,
            settings.esp_settings.box_color,
        );
        let rounding: f32 = settings.esp_settings.box_rounding;

        if settings.esp_settings.enable_boxes {
            if settings.esp_settings.box_enable_shadow {
                let r1 = egui::Rect {
                    min: Pos2 {
                        x: rect.min.x - size,
                        y: rect.min.y - size,
                    },
                    max: Pos2 {
                        x: rect.min.x + size,
                        y: rect.max.y + size,
                    },
                };
                let r2 = egui::Rect {
                    min: Pos2 {
                        x: rect.max.x - size,
                        y: rect.min.y - size,
                    },
                    max: Pos2 {
                        x: rect.max.x + size,
                        y: rect.max.y + size,
                    },
                };
                let r3 = egui::Rect {
                    min: Pos2 {
                        x: rect.min.x - size,
                        y: rect.max.y - size,
                    },
                    max: Pos2 {
                        x: rect.max.x + size,
                        y: rect.max.y + size,
                    },
                };
                let r4 = egui::Rect {
                    min: Pos2 {
                        x: rect.min.x - size,
                        y: rect.min.y - size,
                    },
                    max: Pos2 {
                        x: rect.max.x + size,
                        y: rect.min.y + size,
                    },
                };
                g.add(RectShape::filled(r1, rounding * 2.0, shadow_color).with_blur_width(blur));
                g.add(RectShape::filled(r2, rounding * 2.0, shadow_color).with_blur_width(blur));
                g.add(RectShape::filled(r3, rounding * 2.0, shadow_color).with_blur_width(blur));
                g.add(RectShape::filled(r4, rounding * 2.0, shadow_color).with_blur_width(blur));
            }

            g.rect_stroke(rect, rounding, stroke);
        }

        if settings.esp_settings.enable_health {
            draw_health(
                g,
                rect,
                format!("{}/{}", player.entity.health, player.entity.max_health),
                &settings.esp_settings
            );
        };

        if settings.esp_settings.enable_name {
            draw_name(
                g,
                rect,
                player.name.clone(),
                &settings.esp_settings
            );
        };
    }
}
