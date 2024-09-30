use eframe::egui::Color32;
use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Settings{
    
    //Aimbot
    pub enable_aimbot: bool,
    pub aimbot_fov: f32,
    pub aimbot_smoothing: f32,
    pub aimbot_smoothing_range: i32,

    //ESP
    pub esp_settings: EspSettings,
    //Misc
    pub enable_bhop: bool,

    //Menu
    pub heading_color: Color32,

}

#[derive(Clone, Serialize, Deserialize)]
pub struct EspSettings{
    pub enable_esp: bool,


    //Boxes settings
    pub enable_boxes: bool,
    pub box_size: f32,
    pub box_color: Color32,
    pub box_rounding: f32,
    pub box_enable_shadow: bool,
    pub box_shadow_color: Color32,
    pub box_shadow_blur: f32,
    pub box_shadow_size: f32,

    //Health settings
    pub enable_health: bool,
    pub health_font_size: f32,
    pub health_font_color: Color32,
    pub health_shadow: bool,
    pub health_shadow_color: Color32,

    //Name settings
    pub enable_name: bool,
    pub name_font_size: f32,
    pub name_font_color: Color32,
    pub name_shadow: bool,
    pub name_shadow_color: Color32,

    pub enable_tracer: bool,
}

impl Default for EspSettings {
    fn default() -> Self {
        Self {
            enable_esp: false,
            enable_tracer: false,

            //Boxes settings
            enable_boxes: false,
            box_size: 1.0,
            box_color: Color32::from_rgb(255, 255, 255),
            box_rounding: 1.0,
            box_enable_shadow: false,
            box_shadow_color: Color32::from_rgb(0, 0, 0),
            box_shadow_blur: 1.0,
            box_shadow_size: 1.0,

            //Health settings
            enable_health: false,
            health_font_size: 12.0,
            health_font_color: Color32::from_rgb(255, 255, 255),
            health_shadow: false,
            health_shadow_color: Color32::from_rgb(0, 0, 0),


            //Name settings
            enable_name: false,
            name_font_size: 12.0,
            name_font_color: Color32::from_rgb(255, 255, 255),
            name_shadow: false,
            name_shadow_color: Color32::from_rgb(0, 0, 0),

        }
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            //Aimbot
            enable_aimbot: false,
            aimbot_fov: 0.0,
            aimbot_smoothing: 0.0,
            aimbot_smoothing_range: 0,
            //ESP
            esp_settings: EspSettings::default(),
            //Misc
            enable_bhop: false,
            //Menu
            heading_color: Color32::GREEN,
        }
    }
}