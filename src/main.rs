#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example

pub mod external;
pub mod hacks;
pub mod menu;
pub mod offsets;
pub mod overlay;
pub mod settings;
pub mod utils;

use eframe::{
    egui::{self, Pos2, Vec2},
    Renderer,
};
use external::driver_control::driver::{
    attach_to_process, connect_to_driver, get_module_base,
};
use overlay::overlay::ZyferOverlay;
use utils::{configs::get_configs, process::find_process};
use utils::screen;
use windows::{
    core::s,
    Win32::UI::WindowsAndMessaging::{MessageBoxA, MB_ICONERROR, MB_OK},
};

fn main() -> eframe::Result {
    //try to connect to driver

    let driver_handle = connect_to_driver();

    if driver_handle.is_err() {
        unsafe {
            MessageBoxA(
                None,
                s!("Failed to connect to driver"),
                s!("[Zyfer]"),
                MB_OK | MB_ICONERROR,
            )
        };
        panic!("Failed to connect to driver");
    }

    let driver_handle = driver_handle.unwrap();

    //Attach driver to cs2
    let cs2_pid = unsafe { find_process("cs2.exe") };

    if cs2_pid.is_err() {
        unsafe {
            MessageBoxA(
                None,
                s!("Failed to find cs2 process"),
                s!("[Zyfer]"),
                MB_OK | MB_ICONERROR,
            )
        };
        panic!("Failed to find process");
    }

    let result = attach_to_process(driver_handle, cs2_pid.unwrap());
    if result.is_err() {
        unsafe {
            MessageBoxA(
                None,
                s!("Failed to attach to process"),
                s!("[Zyfer]"),
                MB_OK | MB_ICONERROR,
            )
        };
        panic!("Failed to attach to process");
    }

    //Get module base
    let module_base = get_module_base(driver_handle);

    if module_base.is_null() {
        unsafe {
            MessageBoxA(
                None,
                s!("Failed to get module base"),
                s!("[Zyfer]"),
                MB_OK | MB_ICONERROR,
            )
        };
        panic!("Failed to get module base");
    }

    let monitor = screen::detect();

    env_logger::builder()
        .filter_module("zyfer_cs2_rs", log::LevelFilter::Info)
        .init();

    //check if folder exists
    if !std::path::Path::new("./configs").exists() {
        std::fs::create_dir("./configs").unwrap();
    }

    //list files with .cfg extension and convert to vec
    let config_files = get_configs();
    log::info!("Running...");

    let mut options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_transparent(true)
            .with_always_on_top()
            .with_taskbar(true)
            .with_position(Pos2 {
                x: monitor.0.x,
                y: monitor.0.y,
            })
            .with_inner_size(Vec2 {
                x: monitor.1.x,
                y: monitor.1.y,
            })
            .with_decorations(false),
        ..Default::default()
    };
    options.renderer = Renderer::Glow;
    eframe::run_native(
        "Zyfer Overlay",
        options,
        Box::new(|cc| {
            let mut app = Box::<ZyferOverlay>::default();
            app.module_base = module_base as usize;
            app.driver_handle = driver_handle;
            app.configs = config_files;
            cc.egui_ctx.set_theme(egui::Theme::Dark);
            Ok(app)
        }),
    )
}
