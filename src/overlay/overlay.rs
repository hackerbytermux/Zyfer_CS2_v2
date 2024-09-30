use std::{ffi::CString, path::PathBuf};

use eframe::egui::{self, RichText};
use windows::{core::PCSTR, Win32::{Foundation::{HANDLE, HWND}, Graphics::Gdi::UpdateWindow, UI::WindowsAndMessaging::{FindWindowExA, SetWindowLongA, WINDOW_LONG_PTR_INDEX}}};

use crate::{hacks::cheats::Cheats, menu::menu::{draw_menu_aimbot, draw_menu_esp, draw_menu_misc}, settings::settings::Settings, utils::{keyboard::{Key, KeyState}, math::Vector2, screen::detect}};

pub struct ZyferOverlay {
    initialized: bool,
    hwnd: HWND,
    ui_mode: bool,
    menu_open_key: Key,
    pub module_base: usize,
    pub driver_handle: HANDLE,
    pub settings: Settings,
    pub configs: Vec<PathBuf>,
    pub current_config: String,
    cheat: Cheats,
}

impl Default for ZyferOverlay {
    fn default() -> Self {
        Self {
            initialized: false,
            hwnd: HWND::default(),
            ui_mode: true,
            menu_open_key: Key::new(0x2D),
            module_base: 0,
            driver_handle: HANDLE::default(),
            settings: Settings::default(),
            configs: Vec::new(),
            current_config: String::new(),
            cheat: Cheats::new(),
        }
    }
}

impl ZyferOverlay {
    fn initialize(&mut self) {
        self.hwnd = unsafe {
            let class = PCSTR::null();
            let window_name = CString::new("Zyfer Overlay").unwrap();
            let window = PCSTR(window_name.as_ptr() as *const u8);
            FindWindowExA(HWND::default(), HWND::default(), class, window).unwrap()
        };
        if self.hwnd.0 == std::ptr::null_mut() {
            log::error!("Overlay HWND is invalid");
            panic!("Window handle is invalid");
        }
        log::info!("Overlay window: {:?}", self.hwnd);
        let screen_size = detect();

        //set up cheat
        self.cheat.set_driver_handle(self.driver_handle);
        self.cheat.set_module_base(self.module_base);
        self.cheat.set_screen_size(Vector2{
            x: screen_size.1.x,
            y: screen_size.1.y
        });

        log::info!("Cheat loaded");
        self.initialized = true;
    }

    pub fn activate(&mut self) {
        log::info!("UI enabled");
        unsafe {
            let attributes: i32 = 8i32 | 32i32;
            SetWindowLongA(self.hwnd, WINDOW_LONG_PTR_INDEX(-20), attributes);
            self.ui_mode = true;
            _ = UpdateWindow(self.hwnd);
        }
    }

    pub fn deactive(&mut self) {
        log::info!("UI disabled");
        unsafe {
            let attributes: i32 = 8i32 | 32i32 | 524288i32 | 134217728;
            SetWindowLongA(self.hwnd, WINDOW_LONG_PTR_INDEX(-20), attributes);
            self.ui_mode = false;
            _ = UpdateWindow(self.hwnd);
        }
    }
}

impl eframe::App for ZyferOverlay {
    fn clear_color(&self, _: &egui::Visuals) -> [f32; 4] {
        [0f32, 0f32, 0f32, 0f32]
    }
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if !self.initialized {
            self.initialize();
        }

        let key: &mut Key = &mut self.menu_open_key;
        key.update();
        if key.state == KeyState::Pressed {
            self.ui_mode = !self.ui_mode;
            match self.ui_mode {
                true => self.activate(),
                false => self.deactive(),
            }
        }

        let panel_frame = egui::Frame {
            fill: egui::Color32::TRANSPARENT,
            ..Default::default()
        };
        egui::CentralPanel::default()
            .frame(panel_frame)
            .show(ctx, |ui| {
                self.cheat.update();
                self.cheat.run_cheats(ui, ctx, &self.settings);
                if self.ui_mode {
                    draw_background(ctx, ui);
                    ui.label(
                        RichText::new(" Zyfer CS2 | Pre-Alpha")
                            .text_style(egui::TextStyle::Heading)
                            .color(self.settings.heading_color),
                    );
                    draw_menu_esp(self, ctx, ui);
                    draw_menu_aimbot(self, ctx, ui);
                    draw_menu_misc(self, ctx, ui);
                }
                ctx.request_repaint();
            });
        
    }
}

fn draw_background(ctx: &egui::Context, ui: &mut egui::Ui) {
    let screen_rect = ctx.screen_rect();
    ui.painter().rect_filled(
        screen_rect,
        egui::Rounding::default(),
        egui::Color32::from_rgba_unmultiplied(0, 0, 0, 150),
    );
}
