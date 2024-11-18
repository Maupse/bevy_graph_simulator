use bevy::{app::Update, input::ButtonInput, math::Vec3, prelude::{in_state, AppExtStates, Changed, Commands, Entity, IntoSystemConfigs, IntoSystemSetConfigs, KeyCode, MouseButton, Plugin, Query, Res, ResMut, With}};

use crate::app::build_graph::components::default_vertex;

use super::{components::{Edge, EditorState, GraphInteraction, Vertex}, res::{GraphAssets, MouseCoords, Trees}};
pub struct AddDeleteEditPlugin;
impl Plugin for AddDeleteEditPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
        .init_state::<EditorState>()
        .add_systems(Update, 
            (
                (add_vertex, add_edge).run_if(in_state(EditorState::Add)),
                (delete_vertex, delete_edge).run_if(in_state(EditorState::Delete)),
        ),
        )
        ;
    }
}


fn add_vertex(
    mut commands: Commands,
    mouse_coords: Res<MouseCoords>,
    graph_assets: Res<GraphAssets>,
    mut trees: ResMut<Trees>,
    mouse_input: Res<ButtonInput<MouseButton>>,
) {
    if !mouse_input.just_pressed(MouseButton::Left) {
        return;
    } 
    let mouse_pos = mouse_coords.world;
    println!("Adding vertex at: {}", mouse_pos);
    let entity = commands.spawn(default_vertex(graph_assets, Vec3::new(mouse_pos.x, mouse_pos.y, 0.))).id();
    let kd_tree = &mut trees.kd;
    if !kd_tree.insert(entity, mouse_pos) {
        println!("could not insert the vertex");
    }
    println!("{:?}", kd_tree);
}


fn delete_vertex(
    mut commands: Commands,

) {

}

fn edit_vertex(
    mut commands: Commands,
    q_vertex_interaction: Query<(&GraphInteraction, Entity), (With<Vertex>, Changed<GraphInteraction>)>
) {

}

fn add_edge(
    mut commands: Commands,
) {
    
}

fn delete_edge(
    mut commands: Commands,
    q_edge_interaction: Query<(&GraphInteraction, Entity), (With<Edge>, Changed<GraphInteraction>)>
) {
    
}

fn edit_edge(
    mut commands: Commands,
    q_edge_interaction: Query<(&GraphInteraction, Entity), (With<Edge>, Changed<GraphInteraction>)>
) {
    
}