pub mod app;

#[cfg(target_arch = "wasm32")]
pub mod wasm_module {
    use wasm_bindgen::prelude::*; 
    use crate::app::run;
     
    
    #[wasm_bindgen]
    extern "C" {
        fn alert(s: &str);

        #[wasm_bindgen(js_namespace = console)]
        fn log(s: &str);
    }
    
    #[wasm_bindgen]
    pub fn alert_js(message: &str) {
        alert(message);
    }
    
    #[wasm_bindgen]
    pub fn log_js(message: &str) {
        log(message);
    }
    
    #[wasm_bindgen]
    pub fn run_app() {
        run();
    }
}