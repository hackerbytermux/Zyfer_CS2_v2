use crate::utils::math::Vector3;


pub struct CBaseEntity {
    pub base: usize,

    pub max_health: u32,
    pub health: u32,
    pub team: i32,
    pub life_state: i32,
    pub position: Vector3,
    pub vec_view_offset: Vector3,
    pub cs_player_controller: usize,
}

impl CBaseEntity {
    pub fn new() -> Self {
        Self {
            base: 0,
            max_health: 0,
            health: 0,
            team: 0,
            life_state: 0,
            position: Vector3::default(),
            vec_view_offset: Vector3::default(),
            cs_player_controller: 0,
        }
    }
}

pub struct Player{
    pub entity: CBaseEntity,
    pub name: String,
}