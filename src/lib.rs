pub mod app;

#[cfg(target_arch = "wasm32")]
pub mod wasm_module {
    use wasm_bindgen::prelude::*; 
    use crate::app::run;
     
    
    #[wasm_bindgen]
    extern "C" {
        fn alert(s: &str);
    }
    
    #[wasm_bindgen]
    pub fn greet(name: &str) {
        alert(&format!("Hello, {name}"));
    }
    
    #[wasm_bindgen]
    pub fn run_app() {
        run();
    }
}