use crate::{
    external::driver_control::driver::{read_memory, read_memory_raw}, hacks::esp::draw_esp, offsets::{
        client_dll::cs2_dumper::schemas::client_dll::{
            CBasePlayerController::m_iszPlayerName,
            CCSPlayerController::m_hPlayerPawn,
            C_BaseEntity::{m_iHealth, m_iMaxHealth, m_iTeamNum, m_lifeState},
            C_BaseModelEntity::m_vecViewOffset,
            C_BasePlayerPawn::{m_pObserverServices, m_vOldOrigin},
        },
        offsets::cs2_dumper::offsets::client_dll::{dwEntityList, dwLocalPlayerPawn, dwViewMatrix},
    }, settings::settings::Settings, utils::math::{Vector2, Vector3}
};
use eframe::egui;
use windows::Win32::Foundation::HANDLE;

use super::{spectator_list::draw_spectator_list, structs::{CBaseEntity, Player}};

pub struct Cheats {
    pub module_base: usize,
    pub driver_handle: HANDLE,
    pub screen_size: Vector2,
    pub players: Vec<Player>,
    pub local_player: CBaseEntity,
    pub view_matrix: [f32; 16],
}

macro_rules! check {
    ($var:ident) => {
        if $var == 0 {
            return;
        }
    };
}

macro_rules! con_check {
    ($var:ident) => {
        if $var == 0 {
            continue;
        }
    };
}

impl Cheats {
    pub fn new() -> Self {
        Self {
            module_base: 0,
            driver_handle: HANDLE::default(),
            screen_size: Vector2::default(),
            players: Vec::new(),
            local_player: CBaseEntity::new(),
            view_matrix: [0f32; 16],
        }
    }

    pub fn set_driver_handle(&mut self, handle: HANDLE) {
        self.driver_handle = handle;
    }

    pub fn set_module_base(&mut self, base: usize) {
        self.module_base = base;
    }

    pub fn set_screen_size(&mut self, size: Vector2) {
        self.screen_size = size;
    }

    pub fn run_cheats(&mut self, ui: &mut egui::Ui, ctx: &egui::Context, settings: &Settings) {
        if settings.esp_settings.enable_esp {
            draw_esp(&self, ui, settings);
        }
        draw_spectator_list(&self, ui, ctx, settings);
    }

    fn read_base_entity(&self, address: usize, pawn: usize) -> CBaseEntity {
        let max_health = read_memory(self.driver_handle, address + m_iMaxHealth);
        let health: u32 = read_memory(self.driver_handle, address + m_iHealth);
        let team: i32 = read_memory(self.driver_handle, address + m_iTeamNum);
        let life_state: i32 = read_memory(self.driver_handle, address + m_lifeState);
        let position: Vector3 = read_memory(self.driver_handle, address + m_vOldOrigin);
        let view_offset: Vector3 = read_memory(self.driver_handle, address + m_vecViewOffset);

        let mut cs_player_controller = 0;
        if pawn != 0 {
            cs_player_controller = read_memory(self.driver_handle, pawn as usize + m_pObserverServices);
        }

        CBaseEntity {
            base: address as usize,
            max_health,
            health,
            team,
            life_state: life_state,
            position: position,
            vec_view_offset: view_offset,
            cs_player_controller: cs_player_controller,
        }
    }

    fn read_name(&self, address: usize) -> Result<String, ()> {
        let name: Vec<u8> = read_memory_raw(self.driver_handle, address + m_iszPlayerName, 255);

        if let Some(null_pos) = name.iter().position(|&c| c == 0) {
            let trimmed_vec = &name[..null_pos];

            match String::from_utf8(trimmed_vec.to_vec()) {
                Ok(s) => Ok(s),
                Err(_) => Err(()),
            }
        } else {
            Err(())
        }
    }

    pub fn update(&mut self) {
        self.players.clear();
        let entity_list: usize = read_memory(self.driver_handle, self.module_base + dwEntityList);
        check!(entity_list);

        let list_entity: usize = read_memory(self.driver_handle, entity_list + 0x10);
        check!(list_entity);

        //read view matrix
        let view_matrix = read_memory::<[f32; 16]>(
            self.driver_handle,
            self.module_base + dwViewMatrix,
        );

        self.view_matrix = view_matrix;

        //read local player
        let local_player_pawn: usize =
            read_memory(self.driver_handle, self.module_base + dwLocalPlayerPawn);
        check!(local_player_pawn);

        let local_player = self.read_base_entity(local_player_pawn, 0);
        self.local_player = local_player;

        for i in 0..64 {
            let entity: usize = read_memory(self.driver_handle, list_entity + i * 0x78);
            con_check!(entity);

            let pawn: u32 = read_memory(self.driver_handle, entity + m_hPlayerPawn);

            con_check!(pawn);

            let list_entry2: usize = read_memory(
                self.driver_handle,
                entity_list + ((0x8 * ((pawn & 0x7fff) >> 9) + 0x10) as usize),
            );
            con_check!(list_entry2);

            let current_pawn: usize = read_memory(
                self.driver_handle,
                list_entry2 + (((pawn & 0x1ff) * 0x78) as usize),
            );
            con_check!(current_pawn);

            //read enemy data
            let player = self.read_base_entity(current_pawn, entity);
            let player_name = self.read_name(entity);

            if player_name.is_err() {
                continue;
            }

            self.players.push(Player {
                entity: player,
                name: player_name.unwrap(),
            });
        }
    }
}
