use eframe::egui::{self, Align2, Pos2, Rect};

use crate::{hacks::esp, settings::settings::EspSettings};

pub fn draw_health(
    g: &egui::Painter,
    rect: Rect,
    text: String,
    settings: &EspSettings,
) {
    let mut pos = Pos2 { x: 0., y: 0. };
    let align = Align2::CENTER_TOP;
    pos = Pos2 {
        x: rect.left() + rect.width() / 2.,
        y: rect.bottom()
    };

    let mut font = egui::FontId::default();
    font.size = settings.health_font_size;
    if settings.health_shadow {
        g.text(
            Pos2 {
                x: pos.x + 2.,
                y: pos.y + 2.,
            },
            align,
            text.to_owned(),
            font.clone(),
            settings.health_shadow_color,
        );
    }
    let rect = g.text(pos, align, text, font, settings.health_font_color);
}


pub fn draw_name(
    g: &egui::Painter,
    rect: Rect,
    text: String,
    settings: &EspSettings,
) {
    let mut pos = Pos2 { x: 0., y: 0. };
    let align = Align2::CENTER_TOP;
    pos = Pos2 {
        x: rect.left() + rect.width() / 2.,
        y: rect.top() - 20.,
    };

    let mut font = egui::FontId::default();
    font.size = settings.name_font_size;
    if settings.name_shadow {
        g.text(
            Pos2 {
                x: pos.x + 2.,
                y: pos.y + 2.,
            },
            align,
            text.to_owned(),
            font.clone(),
            settings.name_shadow_color,
        );
    }
    let rect = g.text(pos, align, text, font, settings.name_font_color);
}
