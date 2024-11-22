use std::collections::BinaryHeap;

use bevy::{prelude::{Camera, Entity, GlobalTransform, Handle, Mesh, Query, ResMut, Resource, Vec2, Window, With }, sprite::ColorMaterial, utils::HashMap, window::PrimaryWindow};

use super::kdtree::{TwoDTree};

#[derive(Resource, Default)]
pub struct InputCoords {
    pub world: Option<Vec2>,
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
    pub none_material: Handle<ColorMaterial>,
    pub hovered_material: Handle<ColorMaterial>,
    pub pressed_material: Handle<ColorMaterial>,
}

#[derive(Resource)]
pub struct Trees {
    pub kd: TwoDTree
}

impl Default for Trees {
    fn default() -> Self {
        Self {
            kd: TwoDTree::new(),
        }
    }
}

#[derive(Clone, Copy)]
pub struct DistanceItem(pub Entity, pub f32);

impl Eq for DistanceItem {}

impl PartialEq for DistanceItem {
    fn eq(&self, other: &Self) -> bool {
        self.1 == other.1
    }
}

impl PartialOrd for DistanceItem {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for DistanceItem {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.1.partial_cmp(&other.1).unwrap_or(std::cmp::Ordering::Equal)
    }
}

#[derive(Resource)]
pub struct NearestPoints {
    pub heap: BinaryHeap<DistanceItem>,
}

impl Default for NearestPoints {
    fn default() -> Self {
        Self {
            heap: BinaryHeap::new(),
        }
    }
}