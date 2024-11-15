use bevy::{ecs::system::SystemId, prelude::{Camera, Entity, FromWorld, GlobalTransform, Handle, Has, Mesh, Query, ResMut, Resource, Vec2, Window, With }, sprite::ColorMaterial, utils::HashMap, window::PrimaryWindow};

use super::{add_vertex, delete_vertex, edit_vertex, add_edge, delete_edge, edit_edge};

#[derive(Resource, Default)]
pub struct MouseCoords {
    pub world: Vec2
}

pub fn update_mouse_coords(
    mut mouse_coords: ResMut<MouseCoords>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
) {
    let (camera, glob_camera_transform) = q_camera.single();
    let window = q_window.single();
    let world_pos= window.cursor_position()
        .and_then(|cursor| camera.viewport_to_world(glob_camera_transform, cursor))
        .map(|ray| ray.origin.truncate());

    if world_pos.is_some() {
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
    pub vertex_material: Handle<ColorMaterial>,
    pub edge_material: Handle<ColorMaterial>,
}

#[derive(Resource)]
pub struct GraphSystems {
    pub map: HashMap<String, SystemId>
}

impl FromWorld for GraphSystems {
    fn from_world(world: &mut bevy::prelude::World) -> Self {
        let mut graph_systems = GraphSystems {
            map: HashMap::new()
        };
        graph_systems.map.insert("add_vertex".to_string(), world.register_system(add_vertex));
        graph_systems.map.insert("delete_vertex".to_string(), world.register_system(delete_vertex));
        graph_systems.map.insert("edit_vertex".to_string(), world.register_system(edit_vertex));

        graph_systems.map.insert("add_edge".to_string(), world.register_system(add_edge));
        graph_systems.map.insert("delete_edge".to_string(), world.register_system(delete_edge));
        graph_systems.map.insert("edit_edge".to_string(), world.register_system(edit_edge));
        graph_systems
    }
}

