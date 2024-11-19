use std::{borrow::Borrow, default, process::Command};

use bevy::{app::Plugin, asset::{Assets, Handle}, color::Color, input::ButtonInput, math::Vec3, pbr::graph, prelude::{default, Commands, DespawnRecursiveExt, DetectChanges, Entity, Local, Mesh, MouseButton, Or, Query, Res, ResMut, Transform, Update, With}, sprite::{ColorMaterial, ColorMesh2dBundle, Mesh2dHandle}};

use super::{components::{line_mesh, Edge, GraphInteraction, Vertex}, res::{DistanceItem, GraphAssets, MouseCoords, NearestPoints, Trees}, RADIUS};

pub struct GraphInteractionPlugin; 
impl Plugin for GraphInteractionPlugin {
   fn build(&self, app: &mut bevy::prelude::App) {
       app
       .add_systems(Update, ( update_nearest, update_interactions, color_interactions, lines, graph))
       .init_resource::<NearestPoints>()
       ;
   } 
}

fn update_nearest(
    trees: Res<Trees>,
    mut nearest_points: ResMut<NearestPoints>,
    mouse_coords: Res<MouseCoords>,
) {
    let kd_tree = &trees.kd;
    let mouse_coords = mouse_coords.world;
    if let Some(nearest )= kd_tree.n_nearest_neighboors_search(mouse_coords, 5) {
        nearest_points.heap = nearest;
    }
}

fn update_interactions(
    mut q_interaction: Query<&mut GraphInteraction>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    nearest_points: Res<NearestPoints>,
) {
    for DistanceItem(entity, dist) in nearest_points.heap.iter() {
        if let Ok(mut interaction) = q_interaction.get_mut(*entity) {
            if *dist <= RADIUS {
                if mouse_input.pressed(MouseButton::Left) {
                    *interaction = GraphInteraction::Pressed;
                } else {
                    *interaction = GraphInteraction::Hovered;
                }
            } else {
                *interaction = GraphInteraction::None;
            }
        }
    }
}

fn color_interactions(
    mut q_color: Query<(&GraphInteraction, &mut Handle<ColorMaterial>), Or<(With<Vertex>, With<Edge>)>>,
    materials: Res<GraphAssets>,
    nearest_points: Res<NearestPoints>
) {
    for DistanceItem(entity, _) in nearest_points.heap.iter() {
        let Ok((interaction, mut color_handle)) = q_color.get_mut(*entity) else {continue};
        *color_handle = match interaction {
            GraphInteraction::Hovered => materials.hovered_material.clone(),
            GraphInteraction::Pressed => materials.pressed_material.clone(),
            GraphInteraction::None => materials.none_material.clone(),
        }
    } 
}

fn lines(
    q_transform: Query<&Transform, Or<(With<Vertex>, With<Edge>)>>,
    nearest_points: Res<NearestPoints>,
    mut mesh: Local<Option<Entity>>,
    mut commands: Commands,
    mouse_coords: Res<MouseCoords>,
    graph_assets: Res<GraphAssets>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    if !mouse_coords.is_changed() {
        return;
    }
    if let Some(e) = *mesh {
        commands.entity(e).despawn_recursive();
    }
    
    let mut points: Vec<Vec3> = vec![];
    let m = mouse_coords.world;
    let origin = Vec3::new(m.x, m.y, 0.);
    for DistanceItem(entity, _) in nearest_points.heap.iter() {
        let Ok(transform )= q_transform.get(*entity) else {continue};
        let connection_vector = transform.translation - origin;
        points.push(connection_vector);
    }
    let line_mesh = line_mesh(graph_assets, origin, points, &mut meshes);
    *mesh = Some(commands.spawn(line_mesh).id());
}

fn graph(
    trees: Res<Trees>,
    mut graph: Local<Option<Entity>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    graph_assets: Res<GraphAssets>,
) {
    if !trees.is_changed() {
       return; 
    }
    if let Some(e) = *graph {
        commands.entity(e).despawn_recursive();
    }
    let kd_tree = &trees.kd;
    let Some(tree_mesh )= kd_tree.as_mesh() else {return};

    let color_mesh_2d = ColorMesh2dBundle {
        mesh: Mesh2dHandle::from(meshes.add(tree_mesh)),
        material: graph_assets.none_material.clone(),
        ..default()
    };

    *graph = Some(commands.spawn(color_mesh_2d).id()) 
}
