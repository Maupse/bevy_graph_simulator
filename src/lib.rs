
pub mod app;

#[cfg(target_arch = "wasm32")]
pub mod wasm_module {
    use serde::{Deserialize, Serialize};
    use bevy::{window::WindowResolution};
    use wasm_bindgen::prelude::{wasm_bindgen, JsValue}; 
    use serde_wasm_bindgen::{from_value, Error};
    use crate::app::run;

    #[derive(Serialize, Deserialize)]
    pub struct WindowSize {
        pub width: f32,
        pub height: f32,
    }
     
    impl WindowSize {
        fn from_js_value(js_val: JsValue) -> Result<Self, Error> {
            from_value(js_val)
        }
    }

    #[wasm_bindgen]
    extern "C" {
        fn alert(s: &str);

        #[wasm_bindgen(js_namespace = console)]
        fn log(s: &str);
        
        #[wasm_bindgen(js_namespace = console)]
        fn error(s: &str);
        
        #[wasm_bindgen(js_namespace = window)]
        fn get_window_size() -> JsValue;
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
    
    pub fn get_app_window_size() -> Option<WindowResolution> {
        let js_val = get_window_size();
        let ws = WindowSize::from_js_value(js_val);
        match ws {
            Ok(ws) => Some(WindowResolution::new(ws.width, ws.height)),
            Err(e) => {
                error(&format!("Error serializing the window size {}", e));             
                None
            }
        }
         
    }
}

