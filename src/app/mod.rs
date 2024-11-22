mod build_graph;
mod camera;
mod input;

use bevy::{app::PluginGroup, math::Vec2, prelude::{default, App, DefaultPlugins}, window::{Window, WindowPlugin, WindowResolution}};
use build_graph::BuildGraphPlugin;
use camera::MyCameraPlugin;
use input::MyInputPlugin;
#[cfg(target_arch = "wasm32")]
use crate::wasm_module::log_js;

pub const SCREEN_SIZE: Vec2 = Vec2::new(1280f32, 720f32);
pub fn run() {
    #[cfg(target_arch = "wasm32")]
    log_js("Attempting to run App...");

    let mut app = App::new();
    
    let primary_window = Window {
        resolution: WindowResolution::new(SCREEN_SIZE.x, SCREEN_SIZE.y),
        title: "Bevy Graph Simulator".to_string(),
        canvas: Some("#bevy_graph_simulator_canvas".to_string()),
        ..default()
    };
    
    app
    .add_plugins(DefaultPlugins.set(WindowPlugin{primary_window: Some(primary_window), ..default()}))
    .add_plugins((
        BuildGraphPlugin,
        MyCameraPlugin,
        MyInputPlugin,
    ))
    ;

    #[cfg(not(target_arch = "wasm32"))] 
    {
        use bevy_inspector_egui::quick::WorldInspectorPlugin;
        app.add_plugins(WorldInspectorPlugin::default());
    }
 
    app.run();
    

    #[cfg(target_arch = "wasm32")]
    log_js("Runner was called via app.run()");
}
