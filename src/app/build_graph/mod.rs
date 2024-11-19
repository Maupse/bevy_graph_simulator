mod components;
mod res;
mod graph_interaction;
mod kdtree;
mod add_delete_edit;


use bevy::{app::Startup, asset::Assets, color::Color, prelude::{App, Camera2dBundle, Circle, Commands, Mesh, Plugin, ResMut, Update}, render::camera::CameraPlugin, sprite::ColorMaterial};
use graph_interaction::GraphInteractionPlugin;
use res::{update_mouse_coords, GraphAssets, MouseCoords, Trees};
use add_delete_edit::AddDeleteEditPlugin;

pub struct BuildGraphPlugin;
impl Plugin for BuildGraphPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugins(
            (
                AddDeleteEditPlugin,
                GraphInteractionPlugin,
            )
        )
        .init_resource::<MouseCoords>()
        .init_resource::<Trees>()
        .add_systems(Startup, (init_mesh))
        .add_systems(Update, (
                update_mouse_coords,
       ));
    }
}

pub const RADIUS: f32 = 50.0;

fn init_mesh(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let graph_mesh = GraphAssets {
        vertex: meshes.add(Circle::new(RADIUS)),
        none_material: materials.add(ColorMaterial::from_color(Color::WHITE)),
        hovered_material: materials.add(ColorMaterial::from_color(Color::linear_rgb(1., 0., 0.))),
        pressed_material: materials.add(ColorMaterial::from_color(Color::linear_rgb(0., 1., 0.))),
    };
    
    commands.insert_resource(graph_mesh);
}