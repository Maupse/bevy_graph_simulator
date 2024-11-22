use std::{borrow::Borrow, collections::BinaryHeap, default, process::Command};

use bevy::{app::Plugin, asset::{Assets, Handle}, color::Color, input::ButtonInput, math::Vec3, pbr::graph, prelude::{default, Commands, DespawnRecursiveExt, DetectChanges, Entity, Local, Mesh, MouseButton, Or, Query, Res, ResMut, Transform, Update, With}, sprite::{ColorMaterial, ColorMesh2dBundle, Mesh2dHandle}};
use leafwing_input_manager::prelude::ActionState;

use crate::app::input::NormalInput;

use super::{components::{line_mesh, Edge, GraphInteraction, Vertex}, res::{DistanceItem, GraphAssets, InputCoords, NearestPoints, Trees}, RADIUS};

pub struct GraphInteractionPlugin; 
impl Plugin for GraphInteractionPlugin {
   fn build(&self, app: &mut bevy::prelude::App) {
       app
       .add_systems(Update, ( update_nearest, update_interactions, color_interactions, lines, graph_mesh_system))
       .init_resource::<NearestPoints>()
       ;
   } 
}

fn update_nearest(
    trees: Res<Trees>,
    mut nearest_points: ResMut<NearestPoints>,
    input_coords: Res<InputCoords>,
) {
    let kd_tree = &trees.kd;
    let Some(mouse_coords )= input_coords.world else {return};
    if let Some(nearest )= kd_tree.n_nearest_neighboors_search(mouse_coords, 5) {
        nearest_points.heap = nearest;
    }
}

fn update_interactions(
    mut q_interaction: Query<&mut GraphInteraction>,
    q_action: Query<&ActionState<NormalInput>>,
    nearest_points: Res<NearestPoints>,
    mut last_nearest: Local<BinaryHeap<DistanceItem>>,
) {
    let action = q_action.single();
    for DistanceItem(entity, _) in last_nearest.iter() {
        if let Ok(mut interaction) = q_interaction.get_mut(*entity) {
            *interaction = GraphInteraction::None;
        }
    }

    for DistanceItem(entity, dist) in nearest_points.heap.iter() {
        if let Ok(mut interaction) = q_interaction.get_mut(*entity) {
            if *dist <= RADIUS {
                if action.pressed(&NormalInput::Pressed) {
                    *interaction = GraphInteraction::Pressed;
                } else {
                    *interaction = GraphInteraction::Hovered;
                }
            } else {
                *interaction = GraphInteraction::None;
            }
        }
    }
    *last_nearest = nearest_points.heap.clone();
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
    input_coords: Res<InputCoords>,
    graph_assets: Res<GraphAssets>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    if !input_coords.is_changed() {
        return;
    }
    if let Some(e) = *mesh {
        commands.entity(e).despawn_recursive();
    }
    
    let mut points: Vec<Vec3> = vec![];
    let Some(m) = input_coords.world else {return};

    let origin = Vec3::new(m.x, m.y, 1.);
    for DistanceItem(entity, _) in nearest_points.heap.iter() {
        let Ok(transform )= q_transform.get(*entity) else {continue};
        let connection_vector = transform.translation - origin;
        points.push(connection_vector);
    }
    let line_mesh = line_mesh(graph_assets, origin, points, &mut meshes);
    *mesh = Some(commands.spawn(line_mesh).id());
}

fn graph_mesh_system(
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
        transform: Transform::from_xyz(0., 0., 10.),
        ..default()
    };
    

    *graph = Some(commands.spawn(color_mesh_2d).id()) 
}
