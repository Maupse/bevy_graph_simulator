#[cfg(not(target_arch = "wasm32"))]
mod app;

#[cfg(not(target_arch = "wasm32"))]
use app::run;

pub fn main() {
    #[cfg(not(target_arch = "wasm32"))]
    run();
}

#[cfg(not(target_arch = "wasm32"))]
pub mod not_wasm_module {
    use bevy::{window::WindowResolution};

    pub fn get_app_window_size() -> Option<WindowResolution> {
        Some( WindowResolution::new(1280f32, 720f32) )
    }
}