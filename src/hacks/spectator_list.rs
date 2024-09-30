use egui::RichText;

use crate::{external::driver_control::driver::read_memory, offsets::client_dll::cs2_dumper::schemas::client_dll::{
    CBasePlayerController::m_hPawn, CCSPlayerController::{m_hObserverPawn, m_hPlayerPawn}, CPlayer_ObserverServices::m_hObserverTarget, C_BasePlayerPawn::{m_hController,  m_pObserverServices}}, settings::settings::Settings};

use super::cheats::Cheats;


pub fn draw_spectator_list(cheats: &Cheats, ui: &mut egui::Ui, ctx: &egui::Context, settings: &Settings) {
    println!("SPECTATOR LIST");
    for player in &cheats.players {
        
        let observer_service: u64 = read_memory(cheats.driver_handle, player.entity.base  + m_pObserverServices);
        let observer_target = read_memory::<u64>(cheats.driver_handle, observer_service as usize + m_hObserverTarget);

        if observer_target == 0 {
            continue;
        }

        println!("{} | {}", observer_target, cheats.local_player.base);

    }

    /* 
    egui::Window
    ::new(RichText::new("SPECTATOR LIST").color(settings.heading_color))
    .resizable(false)
    .show(ctx, |ui| {
        ui.group(|ui| {
            ui.vertical(|ui| {
                ui.label(format!("{} spectaros", spectaros.len()));
            })
        })
    });*/
}