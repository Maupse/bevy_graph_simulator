use std::{collections::{BinaryHeap, VecDeque}, ptr::NonNull, sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard}};

use bevy::{math::Vec3, prelude::{Entity, Mesh, Vec2}, render::{mesh::Indices, render_asset::RenderAssetUsages}};

use super::res::DistanceItem;

const DIMENSION: usize = 2;

type RootPointer = Arc<RwLock<TreeNode>>;
type TreeNodePointer = NonNull<TreeNode>;

#[derive(Debug)]
struct TreeNode {
    pub(crate) entity: Entity,
    pub(crate) location: Vec2,
    pub(crate) branch: [Option<TreeNodePointer>; 2],
    pub(crate) depth: usize,
}

impl TreeNode {
    fn new(entity: Entity, point: Vec2, depth: usize) -> Self {
        Self {
            entity: entity,
            location: point,
            branch: [None, None],
            depth: depth,
        }
    }   
    fn from(entity: Entity, point: Vec2, depth: usize) -> Option<TreeNodePointer> {
        let node = TreeNode::new(entity, point, depth);
        let boxed= Box::new(node);
        let raw = Box::into_raw(boxed);
        let nn = NonNull::new(raw);
        nn
    }
    
    fn root_from(entity: Entity, point: Vec2) -> Option<RootPointer> {
        let mutex = RwLock::new(TreeNode::new(entity, point, 0));
        let arc = Arc::new(mutex);
        Some(arc)
    }  
}
#[derive(Debug)]
pub struct TwoDTree {
    root: Option<RootPointer>,    
}

unsafe impl Sync for TwoDTree {}
unsafe impl Send for TwoDTree {}


impl TwoDTree {
    pub fn new() -> Self {
        Self {
            root: None,
        }
    }
    
    #[allow(unused)]
    pub fn insert_list(&mut self, list: Vec<(Entity, Vec2)>) {
            
    }
    
    pub fn insert(&mut self, entity: Entity, point: Vec2) -> bool {
        if self.root.is_none() {
            self.root = TreeNode::root_from(entity, point);
            return true;
        } 
        // Lock the tree
        let root = self.root.as_mut().expect("root should already exist when inserting");
        let Some(mut guard) = Self::get_write_guard(root) else {return false};
        unsafe {
            let (parent, index) = Self::search_parent_mut(&mut guard, point);
            parent.branch[index] = TreeNode::from(entity, point, parent.depth + 1);
        }
        // Guard gets dropped
        return true;
    }



    pub fn n_nearest_neighboors_search(&self, point: Vec2, n: usize) -> Option<BinaryHeap<DistanceItem>> {
        if self.root.is_none() || n == 0 {
            return None;
        }
        let root = self.root.as_ref().expect("n_nearest root should exist");
        let Some(guard) = Self::get_read_guard(root) else {return None};
        unsafe {
            let mut parent_stack = Self::search_parent_list(&guard, point);
            let mut n_nearest: BinaryHeap<DistanceItem> = BinaryHeap::with_capacity(n);
            
            let c_point: [f32; 2] = point.into();

            while let Some(curr_node) = parent_stack.pop() {
                let cut_dim = curr_node.depth % DIMENSION; 
                let curr_loc: [f32; 2] = curr_node.location.into();

                if n_nearest.len() != n {
                    n_nearest.push(DistanceItem(curr_node.entity, curr_node.location.distance(point))); 
                    let unexplored_side = Self::get_unexplored_side(c_point, curr_loc, cut_dim);
                    if let Some(child) = curr_node.branch[unexplored_side] {
                        Self::explore_subtree(child.as_ref(), point, &mut parent_stack);
                    }
                } else {
                    let greatest_nearest_dist = n_nearest.peek().expect("there should be a greatest nearest distance").1;
                    let could_be_in_radius = f32::abs(curr_loc[cut_dim] - c_point[cut_dim]) < greatest_nearest_dist; 
                    if could_be_in_radius {
                        let dist_to_curr = curr_node.location.distance(point);
                        if dist_to_curr < greatest_nearest_dist {
                            n_nearest.pop();
                            n_nearest.push(DistanceItem(curr_node.entity, dist_to_curr));
                        }
                        let unexplored_side = Self::get_unexplored_side(c_point, curr_loc, cut_dim);
                        if let Some(child) = curr_node.branch[unexplored_side] {
                            Self::explore_subtree(child.as_ref(), point, &mut parent_stack);
                        }
                    }
                }
            }
            Some(n_nearest)
        }
    }
    
    fn get_side(point: [f32; 2], current_location: [f32; 2], cut_dimension: usize) -> usize {
        if point[cut_dimension] <= current_location[cut_dimension] {
            0
        } else {
            1
        }
    }
    fn get_unexplored_side(point: [f32; 2], current_location: [f32; 2], cut_dimension: usize) -> usize {
        if point[cut_dimension] <= current_location[cut_dimension] {
            1
        } else {
            0
        }
    }
    
    pub fn _nearest_neighboor_search(&self, point: Vec2) -> Option<(Entity, f32)> {
        if self.root.is_none() {
            return None;
        }
        let root = self.root.as_ref().expect("the root should exist");
        let Some(guard) = Self::get_read_guard(root) else {return None};
        unsafe {
            let mut parent_stack= Self::search_parent_list(&guard, point);

            let Some(mut curr_node) = parent_stack.pop() else {return None};
            let mut nearest_node = curr_node;
            let mut best_dist = nearest_node.location.distance(point);
            let c_point: [f32; 2] = point.into();
            
            loop {
                let cut_dim = curr_node.depth % DIMENSION; 
                let curr_loc: [f32; 2] = curr_node.location.into();
                let could_be_in_radius = f32::abs(curr_loc[cut_dim] - c_point[cut_dim]) <= best_dist; 
                if could_be_in_radius {
                    let dist_to_curr = curr_node.location.distance(point);
                    if dist_to_curr < best_dist {
                        best_dist = dist_to_curr;
                        nearest_node = curr_node;
                    }
                    
                    let unexplored_side = if c_point[cut_dim] <= curr_loc[cut_dim] {1} else {0};
                    if let Some(subtree_root)= curr_node.branch[unexplored_side] {
                        Self::explore_subtree(subtree_root.as_ref(), point, &mut parent_stack);
                    }
                }
                if parent_stack.is_empty() {
                    return Some((nearest_node.entity, best_dist));
                } else {
                    curr_node = parent_stack.pop().expect("the parent stack should still have one element"); 
                }
            }
        }
    } 
    
    
    fn get_read_guard(root: &RootPointer) -> Option<RwLockReadGuard<'_, TreeNode>> {
        let lock = root.read();
        match lock {
            Ok(guard) => Some(guard),
            Err(e) => {
                println!("Tree is locked for reading, {e}");
                None
            }
        }
    }
    
    fn get_write_guard(root: &mut RootPointer) -> Option<RwLockWriteGuard<'_, TreeNode>> {
        let lock = root.write();
        match lock {
            Ok(guard) => Some(guard),
            Err(e) => {
                println!("Tree is locked for writing, {e}");
                None
            }
        } 
    }
    
    unsafe fn search_parent_mut<'a>(guard: &'a mut RwLockWriteGuard<'_, TreeNode>, point: Vec2) -> (&'a mut TreeNode, usize) {
        let root = &mut (**guard);
        let mut curr = root;
        let mut depth = 0;
        loop {
            let cut_dim = depth % DIMENSION;
            
            let c_loc: [f32; 2] = curr.location.into();
            let c_point: [f32; 2] = point.into(); 
            let side = if c_point[cut_dim] <= c_loc[cut_dim] {0} else {1};

            if let Some(mut next) = curr.branch[side] {
               curr = next.as_mut(); 
               depth += 1;
               continue;
            } else {
                return (curr, side);
            }
        }
    }

    
    unsafe fn _search_parent<'a>(guard: &'a RwLockReadGuard<'_, TreeNode>, point: Vec2) -> (&'a TreeNode, usize) {
        let root = &(**guard);
        let mut curr = root;
        let mut depth = 0;
        loop {
            let i = depth % DIMENSION;
            let c_loc: [f32; 2] = curr.location.into();
            let c_point: [f32; 2] = point.into(); 
            let x = if c_point[i] <= c_loc[i] {0} else {1};
            if let Some(next) = curr.branch[x] {
               curr = next.as_ref();
               depth += 1;
               continue;
            } else {
                return (curr, x);
            }
        }
    }
    
    unsafe fn search_parent_list<'a>(guard: &'a RwLockReadGuard<'_, TreeNode>, point: Vec2) -> Vec<&'a TreeNode> {
        let root = &(**guard);
        let mut stack = vec![];
        Self::explore_subtree(root, point, &mut stack);
        stack
    }
    /**
        # Explore Subtree
        This function explores the Tree, always picking the Side that is nearer to the dimension coordinate in given depth.
        The stacks first item will be the given root. 
    */
    unsafe fn explore_subtree<'a>(subtree_root: &'a TreeNode, point: Vec2, stack: &mut Vec<&'a TreeNode>) {
        let mut curr = subtree_root;
        let mut depth = subtree_root.depth;
        loop {
            stack.push(curr);
            let cut_dim  = depth % DIMENSION;
            let c_loc: [f32; 2] = curr.location.into();
            let c_point: [f32; 2] = point.into();
            let side = if c_point[cut_dim] <= c_loc[cut_dim] {0} else {1};
            if let Some(next) = curr.branch[side] {
                let next = next.as_ref();
                curr = next;
                depth += 1;
                continue;
            } else {
                return;
            }
        }
    }
    
    pub fn as_mesh(&self) -> Option<Mesh> {
        if self.root.is_none() {
            return None;
        }
        let Some(guard )= Self::get_read_guard(self.root.as_ref().expect("unreachable")) else {return None};
        let root = &(*guard);

        let mut vertices: Vec<Vec3> = vec![];
        let mut indices: Vec<u32> = vec![];

        const Z: f32 = 0.0;
        const INFINITY: f32 = f32::MAX;
        const NEGATIVE_INFINITY: f32 = f32::MIN;

        let mut i = 0 as u32;
        let mut new_line = |x1,  y1, x2, y2| {
            vertices.push(Vec3::new(x1, y1, Z));
            vertices.push(Vec3::new(x2, y2, Z));
            indices.push(i);
            indices.push(i + 1);
            i += 2;
        };

        // exploring left you update either east or south
        // exploring right you update either west or north
        let limit_index = |cut_dim: usize, explore: usize| -> usize {
            explore * 2 + cut_dim
        };

        const LEFT: usize = 0;
        const RIGHT: usize = 1;
        const X: usize = 0;
        const Y: usize = 1;
        
        const E: usize = 0usize;
        const N: usize = 1usize;
        const W: usize = 2usize;
        const S: usize = 3usize;

        // [east, north, west, south]
        let initial_limit = [INFINITY, INFINITY, NEGATIVE_INFINITY, NEGATIVE_INFINITY];
        let mut queue = VecDeque::new();
        queue.push_back((root, initial_limit));

        unsafe {
            while !queue.is_empty() {
                let mut next_iteration: VecDeque<(&TreeNode, [f32; 4])>= VecDeque::new();
                for (node, limit) in queue {
                    let cut_dim = node.depth % DIMENSION;
                    let location: [f32; 2] = node.location.into();
                    
                    match cut_dim {
                        X => new_line(location[X], limit[S], location[X], limit[N]),
                        Y => new_line(limit[W], location[Y], limit[E], location[Y]),
                        _ => panic!("a dimension higher than 1 should be unreachable"),
                    }
                    
                    for explore in LEFT..=RIGHT {
                        if let Some(child) = node.branch[explore] {
                            let child = child.as_ref();
                            let mut new_limit = limit.clone();
                            let l = limit_index(cut_dim, explore); 
                            new_limit[l] = location[cut_dim];
                            next_iteration.push_back((child, new_limit));
                        }
                    } 
                }
                queue = next_iteration;
            }
        } 

        let mut mesh = Mesh::new(bevy::render::mesh::PrimitiveTopology::LineList, RenderAssetUsages::RENDER_WORLD);
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices); 
        mesh.insert_indices(Indices::U32(indices));

        Some(mesh)
    }

}