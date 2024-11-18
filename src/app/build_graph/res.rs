use bevy::{color::Color, prelude::{Camera, Entity, GlobalTransform, Handle, Mesh, Query, ResMut, Resource, Vec2, Window, With }, sprite::ColorMaterial, utils::HashMap, window::PrimaryWindow};

use super::kdtree::TwoDTree;

#[derive(Resource, Default)]
pub struct MouseCoords {
    pub world: Vec2
}

pub fn update_mouse_coords(
    mut mouse_coords: ResMut<MouseCoords>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
) {
    let Ok(window)= q_window.get_single() else {return};
    let (camera, glob_camera_transform) = q_camera.single();
    let world_pos= window.cursor_position()
        .and_then(|cursor| camera.viewport_to_world(glob_camera_transform, cursor))
        .map(|ray| ray.origin.truncate());

    if world_pos.is_some() && world_pos.unwrap() != mouse_coords.world {
        mouse_coords.world = world_pos.unwrap();
    }
}

//Maps Vertex -> (AdjVertex, Weight)
#[derive(Resource, Default)]
pub struct AdjacencyList {
    pub map: HashMap<Entity, Vec<(Entity, i32)>>,
}

//Maps (StartVertex, DestVertex) -> Edge
#[derive(Resource, Default)]
pub struct EdgeMapping {
    pub map: HashMap<(Entity, Entity), Entity>,
}

#[derive(Resource)]
pub struct GraphAssets {
    pub vertex: Handle<Mesh>,
    pub none_material: Handle<ColorMaterial>,
    pub hovered_material: Handle<ColorMaterial>,
    pub pressed_material: Handle<ColorMaterial>,
}

#[derive(Resource)]
pub struct Trees {
    pub kd: TwoDTree
}

impl Default for Trees {
    fn default() -> Self {
        Self {
            kd: TwoDTree::new(),
        }
    }
}

#[derive(Resource)]
pub struct NearestPoints {
    pub vec: Vec<Entity>,
}

impl Default for NearestPoints {
    fn default() -> Self {
        Self {
            vec: vec![],
        }
    }
}