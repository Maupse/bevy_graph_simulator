mod components;
mod res;

use std::process::{Command, CommandArgs};

use bevy::{app::Startup, asset::Assets, color::Color, input::ButtonInput, math::Vec3, prelude::{App, AppExtStates, Changed, Circle, Commands, Entity, FromWorld, Interaction, Mesh, MouseButton, Plugin, Query, Res, ResMut, State, Update, With}, sprite::ColorMaterial};
use components::{default_vertex, Edge, EditorState, GraphComponentBundle, Vertex};
use res::{update_mouse_coords, GraphAssets, GraphSystems, MouseCoords};

pub struct BuildGraphPlugin;
impl Plugin for BuildGraphPlugin {
    fn build(&self, app: &mut App) {
        app
        .init_resource::<MouseCoords>()
        .init_resource::<GraphSystems>()
        .init_state::<EditorState>()
        .add_systems(Startup, init_mesh)
        .add_systems(Update, (
                update_mouse_coords,
                interaction_listener,      
        ));
    }
}

fn init_mesh(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let graph_mesh = GraphAssets {
        vertex: meshes.add(Circle::new(50.0)),
        vertex_material: materials.add(ColorMaterial::from_color(Color::WHITE)),
        edge_material: materials.add(ColorMaterial::default()), 
    };
    
    commands.insert_resource(graph_mesh);
}


fn interaction_listener(
    mouse_coords: Res<MouseCoords>,
    state: Res<State<EditorState>> ,
    q_vertex_interaction: Query<(&Interaction, Entity), (With<Vertex>, Changed<Interaction>)>,
    q_edge_interaction: Query<(&Interaction, Entity), (With<Edge>, Changed<Interaction>)>,
    mut commands: Commands,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    graph_systems: Res<GraphSystems>,
) {
    let curr_state = state.get();
    let sys = &graph_systems.map;
    match curr_state {
        EditorState::Add => {
           if mouse_buttons.just_released(MouseButton::Left) {
                if q_vertex_interaction.is_empty() {
                    commands.run_system(sys["add_vertex"]);
                }
           }
        },
        EditorState::Edit => {
            if q_edge_interaction.is_empty() && q_edge_interaction.is_empty() {
                return;
            }
            if !q_vertex_interaction.is_empty() {
                return;
            } else {
                return;
            }
        },
        EditorState::Delete => {
            if q_edge_interaction.is_empty() && q_vertex_interaction.is_empty() {
                return
            }
            if !q_vertex_interaction.is_empty() {
                return;
            } else {
                return;
            }

        }
    }
}

fn add_vertex(
    mut commands: Commands,
    mouse_coords: Res<MouseCoords>,
    graph_assets: Res<GraphAssets>,
) {
    let mouse_pos = mouse_coords.world;
    println!("Adding vertex at: {}", mouse_pos);
    commands.spawn(default_vertex(graph_assets, Vec3::new(mouse_pos.x, mouse_pos.y, 0.)));
}

fn delete_vertex(
    mut commands: Commands,

) {

}

fn edit_vertex(
    mut commands: Commands,
    q_vertex_interaction: Query<(&Interaction, Entity), (With<Vertex>, Changed<Interaction>)>
) {

}

fn add_edge(
    mut commands: Commands,
) {
    
}

fn delete_edge(
    mut commands: Commands,
    q_edge_interaction: Query<(&Interaction, Entity), (With<Edge>, Changed<Interaction>)>
) {
    
}

fn edit_edge(
    mut commands: Commands,
    q_edge_interaction: Query<(&Interaction, Entity), (With<Edge>, Changed<Interaction>)>
) {
    
}