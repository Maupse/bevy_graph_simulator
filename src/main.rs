#[cfg(not(target_arch = "wasm32"))]
mod app;

#[cfg(not(target_arch = "wasm32"))]
use app::run;

pub fn main() {
    #[cfg(not(target_arch = "wasm32"))]
    run();
}
