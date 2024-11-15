use bevy::{math::Vec3, prelude::{default, Bundle, Component, Mesh, Res, States, Transform }, sprite::{ColorMesh2dBundle, Mesh2dHandle}, ui::{Interaction, Node}};

use super::res::GraphAssets;

#[derive(Default, States, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EditorState {
    #[default]
    Add,
    Edit,
    Delete,
}

#[derive(Component)]
pub struct Vertex;

#[derive(Component)]
pub struct Edge;

#[derive(Component)]
pub enum GraphInteraction {
    None,
    Hovered,
    Pressed,
}

//Node acts as an identifier for the UI system to query over it and change the interaction state
#[derive(Bundle)]
pub struct GraphComponentBundle {
    graph_interaction: GraphInteraction,    
    color_mesh_bundle: ColorMesh2dBundle,
}

pub fn default_vertex(
    graph_mesh: Res<GraphAssets>,
    pos: Vec3,
) -> (Vertex, GraphComponentBundle) {
    (
        Vertex,
        GraphComponentBundle {
            graph_interaction: GraphInteraction::None,
            color_mesh_bundle: ColorMesh2dBundle {
                mesh: Mesh2dHandle::from(graph_mesh.vertex.clone()),
                material: graph_mesh.vertex_material.clone(),
                transform: Transform {
                    translation: pos,
                    ..default()
                },
                ..default()
            }
        }
    )    
}