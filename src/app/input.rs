use bevy::{app::{PreStartup, Update}, input::ButtonInput, math::Vec2, prelude::{App, Camera, Commands, DetectChanges, Entity, EventReader, GlobalTransform, IntoSystemConfigs, Local, MouseButton, Plugin, Query, Res, ResMut, Resource, Startup, TouchInput, With}, reflect::Reflect, time::Time, utils::HashMap, window::{PrimaryWindow, Window}};
use leafwing_input_manager::{plugin::InputManagerPlugin, prelude::{serde_typetag, ActionState, DualAxislike, InputMap, MouseMove, MouseScrollAxis, UserInput}, Actionlike, InputManagerBundle};

use super::{build_graph::res::InputCoords, camera::{spawn_camera, MainCamera}};

#[cfg(target_arch = "wasm32")]
use crate::wasm_module::log_js;

#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub enum NormalInput {
    Pressed,
    Select,
}

#[derive(Actionlike, Clone, Debug, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum CameraMovement {
    #[actionlike(Axis)]
    Zoom,
}
pub struct MyInputPlugin;
impl Plugin for MyInputPlugin {
   fn build(&self, app: &mut bevy::prelude::App) {
       app
        .init_resource::<InputCoords>()
       .add_plugins((
            InputManagerPlugin::<CameraMovement>::default(),    
            InputManagerPlugin::<NormalInput>::default(),
       ))
       .add_systems(PreStartup, (
           map_camera_input.after(spawn_camera),
           map_action_input,
       ))
    .add_systems(Update, (
        handle_selection,
        handle_touch_input,
        update_mouse_coords,
       ))
       ;
   } 
}

fn map_camera_input(
    mut commands: Commands,
    q_camera: Query<Entity, With<MainCamera>>,
) {
    let input_map = InputMap::default()
    .with_axis(CameraMovement::Zoom, MouseScrollAxis::Y)
    ;
    let e = q_camera.single();
    commands.entity(e).insert(InputManagerBundle::with_map(input_map));
}

fn map_action_input(
    mut commands: Commands,
) {
    let mut input_map = InputMap::default();

    input_map.insert(NormalInput::Pressed, MouseButton::Left);

    commands.spawn(InputManagerBundle::with_map(input_map));
}

pub fn update_mouse_coords(
    mut input_coords: ResMut<InputCoords>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
) {
    let Ok(window)= q_window.get_single() else {return};
    let Ok((camera, glob_camera_transform)) = q_camera.get_single() else {return};
    let world_pos= window.cursor_position()
        .and_then(|cursor| camera.viewport_to_world_2d(glob_camera_transform, cursor));

    if world_pos.is_some() && world_pos != input_coords.world {
        input_coords.world = world_pos;
    }
}


fn handle_selection(
    mut q_myaction: Query<&mut ActionState<NormalInput>>,
    mut pressed_duration: Local<Option<f32>>,
    time: Res<Time>,
) {
    const TRIGGERMAXTIME: f32 = 0.5;
    let mut state = q_myaction.single_mut();
    if state.just_pressed(&NormalInput::Pressed) {
        *pressed_duration = Some(0.);
    } else if state.pressed(&NormalInput::Pressed) {
        let val = pressed_duration.get_or_insert(0.);
        *val += time.delta_seconds();
        println!("pressed with {} seconds", *val);
        #[cfg(target_arch = "wasm32")]
        {
            log_js(&format!("pressed with {} seconds", *val));
            if *val > 5.0 {
                log_js("Hold is longer than 5s");
            }
        }
    } else if state.just_released(&NormalInput::Pressed) {
        if pressed_duration.is_some() && pressed_duration.unwrap() <= TRIGGERMAXTIME {
            println!("Press Select");
            #[cfg(target_arch = "wasm32")]
            {
                log_js("Select detected");
            }
            state.press(&NormalInput::Select);
        }
    } else {
        state.release(&NormalInput::Select);
    }

}

fn handle_touch_input(
    mut q_myaction: Query<&mut ActionState<NormalInput>>,
    mut touch_evr: EventReader<TouchInput>,
    mut input_pos: ResMut<InputCoords>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
) {
    use bevy::input::touch::TouchPhase;

    if touch_evr.is_empty() {
        return;
    }

    let mut state = q_myaction.single_mut();
    let Ok((camera, camera_transform)) = q_camera.get_single() else {return};

    for ev in touch_evr.read() {
        match ev.phase {
            TouchPhase::Started => { 
                input_pos.world = camera.viewport_to_world_2d(camera_transform, ev.position);
                state.press(&NormalInput::Pressed);
            },
            TouchPhase::Moved => {
                input_pos.world = camera.viewport_to_world_2d(camera_transform, ev.position);
                #[cfg(target_arch = "wasm32")]
                {
                    log_js( &format!("Moving touch with id {} and position {} converted to world {:?}", ev.id, ev.position, input_pos.world));
                }
            },
            TouchPhase::Ended => {
                input_pos.world = camera.viewport_to_world_2d(camera_transform, ev.position);
                state.release(&NormalInput::Pressed);
            },
            TouchPhase::Canceled => {
                input_pos.world = None;
                state.release(&NormalInput::Pressed);
            },
        }
    }
}