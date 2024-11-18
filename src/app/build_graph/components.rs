use bevy::{asset::{Assets, Handle}, math::Vec3, prelude::{default, Bundle, Component, Mesh, Res, ResMut, States, Transform }, render::{mesh::{Indices, PrimitiveTopology}, render_asset::RenderAssetUsages}, sprite::{ColorMaterial, ColorMesh2dBundle, MaterialMesh2dBundle, Mesh2dHandle}, ui::{Interaction, Node}};

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
    graph_assets: Res<GraphAssets>,
    pos: Vec3,
) -> (Vertex, GraphComponentBundle) {
    (
        Vertex,
        GraphComponentBundle {
            graph_interaction: GraphInteraction::None,
            color_mesh_bundle: ColorMesh2dBundle {
                mesh: Mesh2dHandle::from(graph_assets.vertex.clone()),
                material: graph_assets.none_material.clone(),
                transform: Transform {
                    translation: pos,
                    ..default()
                },
                ..default()
            }
        }
    )    
}

pub fn line_mesh(
    graph_assets: Res<GraphAssets>,
    origin: Vec3,
    points: Vec<Vec3>,
    meshes: &mut ResMut<Assets<Mesh>>,
) -> ColorMesh2dBundle {
    let mut vertices: Vec<Vec3>= vec![];
    let mut indices: Vec<u32> = vec![];
    let mut i = 0 as u32;

    for p in points {
        vertices.push(Vec3::ZERO);
        vertices.push(p);
        indices.push(i);
        indices.push(i + 1);
        i += 2;
    }
    
    let mut mesh = Mesh::new(PrimitiveTopology::LineList, RenderAssetUsages::RENDER_WORLD);
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices); 
    mesh.insert_indices(Indices::U32(indices));

    ColorMesh2dBundle {
        mesh: Mesh2dHandle::from(meshes.add(mesh)),
        material: graph_assets.none_material.clone(),
        transform: Transform::from_translation(origin),
        ..default()
    }
}