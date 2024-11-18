mod build_graph;

use bevy::{app::{AppExit, PluginGroup, Startup, Update}, prelude::{default, App, Camera2dBundle, Commands, DefaultPlugins, EventReader, EventWriter, Query, TextStyle, With}, text::{Text, Text2dBundle, TextSection}, window::{PrimaryWindow, Window, WindowClosed, WindowPlugin, WindowResolution}};
use build_graph::BuildGraphPlugin;

pub fn run() {
    let mut app = App::new();
    
    let primary_window = Window {
        resolution: WindowResolution::new(1280f32, 720f32),
        title: "Bevy Graph Simulator".to_string(),
        canvas: Some("#bevy_graph_simulator_canvas".to_string()),
        ..default()
    };
    
    app
    .add_systems(Startup, spawn_camera)
    .add_plugins(DefaultPlugins.set(WindowPlugin{primary_window: Some(primary_window), ..default()}));


    #[cfg(not(target_arch = "wasm32"))] 
    {
        use bevy_inspector_egui::quick::WorldInspectorPlugin;
        app.add_plugins(WorldInspectorPlugin::default());
    }
     
    #[cfg(target_arch = "wasm32")]
    {
        use crate::wasm_module::greet;
        greet("from bevy wasm build");
    }
    
    app.add_plugins(
        (
            BuildGraphPlugin,
        )
    );
    
    app.run();
}

fn spawn_camera(
    mut commands: Commands
) {
    commands.spawn(Camera2dBundle::default());
}