use bevy::{app::Update, prelude::{Component, Camera2dBundle, Commands, Plugin, Startup}};
pub struct MyCameraPlugin;
impl Plugin for MyCameraPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
        .add_systems(Startup, spawn_camera)
        .add_systems(Update, camera_zoom) 
        ;
    }    
}

#[derive(Component)]
struct MainCamera;

fn spawn_camera(
    mut commands: Commands
) {
    commands.spawn((Camera2dBundle::default(), MainCamera));
}


fn camera_zoom() {

}