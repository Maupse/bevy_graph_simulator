use std::{collections::BinaryHeap, ptr::NonNull, sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard}, thread::panicking};

use bevy::{prelude::{Entity, Mesh, Vec2}, render::render_asset::RenderAssetUsages, sprite::ColorMesh2dBundle};

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

struct Item(Entity, f32);

impl Eq for Item {}

impl PartialEq for Item {
    fn eq(&self, other: &Self) -> bool {
        self.1 == other.1
    }
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.1.partial_cmp(&other.1).unwrap_or(std::cmp::Ordering::Equal)
    }
}


impl TwoDTree {
    pub fn new() -> Self {
        Self {
            root: None,
        }
    }
    
    pub fn _insert_list(&mut self, list: Vec<(Entity, Vec2)>) {
            
    }
    
    pub fn insert(&mut self, entity: Entity, point: Vec2) -> bool {
        if self.root.is_none() {
            self.root = TreeNode::root_from(entity, point);
            return true;
        } 
        // Lock the tree
        let root = self.root.as_mut().expect("unreachable");
        let Some(mut guard) = Self::get_write_guard(root) else {return false};
        unsafe {
            let (parent, index) = Self::search_parent_mut(&mut guard, point);
            parent.branch[index] = TreeNode::from(entity, point, parent.depth + 1);
        }
        // Guard gets dropped
        return true;
    }



    pub fn n_nearest_neighboors_search(&self, point: Vec2, n: usize) -> Option<BinaryHeap<Item>> {
        if self.root.is_none() {
            return None;
        }
        let root = self.root.as_ref().expect("unreachable");
        let Some(guard) = Self::get_read_guard(root) else {return None};
        unsafe {
            let mut parent_stack = Self::search_parent_list(&guard, point);
            let mut n_nearest: BinaryHeap<Item> = BinaryHeap::with_capacity(n);
            
            let Some(mut curr_node) = parent_stack.pop() else {return None};
            let mut nearest_node = curr_node;
            n_nearest.push(Item(nearest_node.entity, nearest_node.location.distance(point))); 
            let c_point: [f32; 2] = point.into();

            loop {
                if parent_stack.is_empty() {
                    return Some(n_nearest);
                } else {
                    curr_node = parent_stack.pop().expect("unreachable"); 
                }
                let cut_dim = curr_node.depth % DIMENSION; 
                let curr_loc: [f32; 2] = curr_node.location.into();

                let greatest_nearest_dist = n_nearest.peek().expect("unreachable").1;
                let could_be_in_radius = f32::abs(curr_loc[cut_dim] - c_point[cut_dim]) < greatest_nearest_dist; 
            }
            return None;
        }
    }
    
    pub fn nearest_neighboor_search(&self, point: Vec2) -> Option<(Entity, f32)> {
        if self.root.is_none() {
            return None;
        }
        let root = self.root.as_ref().expect("unreachable");
        let Some(guard) = Self::get_read_guard(root) else {return None};
        unsafe {
            let mut parent_stack= Self::search_parent_list(&guard, point);

            let Some(mut curr_node) = parent_stack.pop() else {return None};
            let mut nearest_node = curr_node;
            let mut best_dist = nearest_node.location.distance(point);
            let c_point: [f32; 2] = point.into();
            
            loop {
                if parent_stack.is_empty() {
                    return Some((nearest_node.entity, best_dist));
                } else {
                    curr_node = parent_stack.pop().expect("unreachable"); 
                }
                let cut_dim = curr_node.depth % DIMENSION; 
                let curr_loc: [f32; 2] = curr_node.location.into();
                let could_be_in_radius = f32::abs(curr_loc[cut_dim] - c_point[cut_dim]) < best_dist; 
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

    
    unsafe fn search_parent<'a>(guard: &'a RwLockReadGuard<'_, TreeNode>, point: Vec2) -> (&'a TreeNode, usize) {
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
        let mut stack = vec![root];
        Self::explore_subtree(root, point, &mut stack);
        stack
    }

    unsafe fn explore_subtree<'a>(subtree_root: &'a TreeNode, point: Vec2, stack: &mut Vec<&'a TreeNode>) {
        let mut curr = subtree_root;
        let mut depth = subtree_root.depth;
        loop {
            let cut_dim  = depth % DIMENSION;
            let c_loc = [curr.location.x, curr.location.y];
            let c_point = [point.x, point.y];
            let side = if c_point[cut_dim] <= c_loc[cut_dim] {0} else {1};
            if let Some(next) = curr.branch[side] {
                curr = next.as_ref();
                depth += 1;
                stack.push(curr);
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

        let mesh = Mesh::new(bevy::render::mesh::PrimitiveTopology::LineList, RenderAssetUsages::RENDER_WORLD);
        
        Some(mesh)
    }

}