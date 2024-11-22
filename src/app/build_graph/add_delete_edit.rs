use bevy::{app::Update, math::Vec3, prelude::{in_state, AppExtStates, Changed, Commands, Entity, IntoSystemConfigs, Plugin, Query, Res, ResMut, With}};
use leafwing_input_manager::prelude::ActionState;

use crate::app::{build_graph::components::default_vertex, input::{NormalInput}};

use super::{components::{Edge, EditorState, GraphInteraction, Vertex}, res::{GraphAssets, InputCoords, Trees}};

#[cfg(target_arch = "wasm32")]
use crate::wasm_module::log_js;
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
    graph_assets: Res<GraphAssets>,
    mut trees: ResMut<Trees>,
    input_pos: Res<InputCoords>,
    q_my_action: Query<&ActionState<NormalInput>>,
) {
    let my_action = q_my_action.single();
    if !my_action.just_pressed(&NormalInput::Select) {
        return;
    } 

    #[cfg(target_arch = "wasm32")] 
    log_js("just pressed select");
    println!("just pressed select");
    
    
    let Some( position ) = input_pos.world else {return};

    let entity = commands.spawn(default_vertex(graph_assets, Vec3::new(position.x, position.y, 0.))).id();
    let kd_tree = &mut trees.kd;
    if !kd_tree.insert(entity, position) {
        println!("could not insert the vertex");
    }
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