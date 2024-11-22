use bevy::{app::{Main, PreStartup, Update}, prelude::{Camera2dBundle, Commands, Component, OrthographicProjection, Plugin, Query, Res, Startup, With}, time::Time};
use leafwing_input_manager::prelude::ActionState;

use super::input::CameraMovement;
pub struct MyCameraPlugin;
impl Plugin for MyCameraPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
        .add_systems(PreStartup, spawn_camera)
        .add_systems(Update, camera_zoom) 
        ;
    }    
}

#[derive(Component)]
pub struct MainCamera;

pub fn spawn_camera(
    mut commands: Commands
) {
    commands.spawn((Camera2dBundle::default(), MainCamera));
}


fn camera_zoom(
    mut q_cam: Query<(&mut OrthographicProjection, &ActionState<CameraMovement>), With<MainCamera>>,
    time: Res<Time>,
) {
    const CAMERA_ZOOM_RATE: f32 = 12f32;

    let Ok((mut proj, state)) = q_cam.get_single_mut() else {return};

    let zoom_delta = state.value(&CameraMovement::Zoom);
    
    proj.scale = (proj.scale * 1f32 - time.delta_seconds() *  zoom_delta * CAMERA_ZOOM_RATE).clamp(1.4f32, 18f32);
}