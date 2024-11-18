use std::{borrow::Borrow, process::Command};

use bevy::{app::Plugin, asset::{Assets, Handle}, color::Color, input::ButtonInput, math::Vec3, prelude::{Changed, Commands, DespawnRecursiveExt, DetectChanges, Entity, Local, Mesh, MouseButton, Or, Query, Res, ResMut, Transform, Update, With}, sprite::ColorMaterial};

use super::{components::{line_mesh, Edge, GraphInteraction, Vertex}, res::{GraphAssets, MouseCoords, NearestPoints, Trees}, RADIUS};

pub struct GraphInteractionPlugin; 
impl Plugin for GraphInteractionPlugin {
   fn build(&self, app: &mut bevy::prelude::App) {
       app
       .add_systems(Update, ( update_interactions, color_interactions, lines))
       .init_resource::<NearestPoints>()
       ;
   } 
}

fn update_interactions(
    mut q_interaction: Query<&mut GraphInteraction>,
    mouse_coords: Res<MouseCoords>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    trees: Res<Trees>,
    mut nearest_points: ResMut<NearestPoints>,
) {
    let kd_tree = &trees.kd;
    let mouse_pos = mouse_coords.world;
    let nearest = kd_tree.nearest_neighboor_search(mouse_pos);
    if let Some((entity, dist)) = nearest {
        nearest_points.vec = vec![entity];
        if let Ok(mut interaction) = q_interaction.get_mut(entity) {
            if dist <= RADIUS {
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
    for entity in nearest_points.vec.iter() {
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
    for nearest in nearest_points.vec.clone() {
        let Ok(transform )= q_transform.get(nearest) else {continue};
        let connection_vector = transform.translation - origin;
        points.push(connection_vector);
    }
    let line_mesh = line_mesh(graph_assets, origin, points, &mut meshes);
    *mesh = Some(commands.spawn(line_mesh).id());
}