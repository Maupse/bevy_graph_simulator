mod build_graph;

use bevy::{app::{PluginGroup, Startup}, prelude::{default, App, Camera2dBundle, Commands, DefaultPlugins, TextStyle}, text::{Text, Text2dBundle, TextSection}, window::{Window, WindowPlugin, WindowResolution}};
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
    .add_systems(Startup, (
        hello_world,
    ))
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

fn hello_world(
    mut commands: Commands,
    
) {
    println!("Hello World");
    let hello_world_bundle = Text2dBundle {
       text: Text {
            sections: vec![TextSection::new("Hello World", TextStyle::default())],
            ..default()
       },
       ..default()
    };
    commands.spawn(Camera2dBundle::default());
    commands.spawn(hello_world_bundle);
}