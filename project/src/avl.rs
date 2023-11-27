// use std::borrow::Borrow;
// use std::cmp::{max, Ordering};
// use std::fmt::Debug;
// use std::mem::{replace, swap};
// use std::option::Option::Some;

// #[derive(Debug, PartialEq, Clone)]
// pub struct AvlNode<T: Ord> {
//     // Node data structure
//     pub value: T,
//     pub left: AvlTree<T>,
//     pub right: AvlTree<T>,
//     pub height: usize, // The Depth of each node
// }

// type BoxAvlNode<T> = Option<Box<AvlNode<T>>>;

// #[derive(Debug, PartialEq, Clone)]
// pub struct AvlTree<T: Ord>(pub BoxAvlNode<T>);
// //Tree data structure

// impl<'a, T: 'a + Ord + Debug> AvlNode<T> {
//     pub fn count_leaves(&self) -> usize {
//         if let Some(left_child) = &self.left.0 {
//             if let Some(right_child) = &self.right.0 {
//                 return (*left_child).count_leaves() + (*right_child).count_leaves();
//             } else {
//                 return (*left_child).count_leaves();
//             }
//         } else {
//             if let Some(right_child) = &self.right.0 {
//                 return (*right_child).count_leaves();
//             } else {
//                 return 1;
//             }
//         }
//     }

//     pub fn left_height(&self) -> usize {
//         self.left.0.as_ref().map_or(0, |left| left.height())
//     }

//     pub fn right_height(&self) -> usize {
//         self.right.0.as_ref().map_or(0, |right| right.height())
//     }

//     pub fn height(&self) -> usize {
//         1 + max(self.left_height(), self.right_height())
//         //use recursive to calculate height the larger height of the left or right subtree plus one
//     }

//     pub fn update_height(&mut self) {
//         self.height = 1 + max(self.left_height(), self.right_height());
//     }

//     pub fn balance_factor(&self) -> i8 {
//         //calculate balance factor, which means the difference of height between left child and right child
//         let left_height = self.left.0.as_ref().map_or(0, |left| left.height());
//         let right_height = self.right.0.as_ref().map_or(0, |right| right.height());

//         if left_height >= right_height {
//             (left_height - right_height) as i8
//         } else {
//             -((right_height - left_height) as i8)
//             //prevent internal cycle
//         }
//     }

//     pub fn rotation_right(&mut self) -> bool {
//         //After inserting a node, the tree may be imbalance, so we need to rebalance it throughh rotation

//         if self.left.0.is_none() {
//             return false;
//             //This situation can't rotate
//         }

//         let left_node = self.left.0.as_mut().unwrap();
//         let left_right_node = left_node.right.0.take();
//         let left_left_node = left_node.left.0.take();
//         //Take the left child and right child of the current node's left node

//         let mut new_right_tree = replace(&mut self.left.0, left_left_node);
//         //The left node of the current node will become the root node after rotation, so the left node's left
//         //node will become the left node.

//         swap(&mut self.value, &mut new_right_tree.as_mut().unwrap().value);
//         //swap the value of the current and the left node

//         let right_tree = self.right.0.take();
//         //take the right node, and now there isn't right node

//         let new_right_node = new_right_tree.as_mut().unwrap();
//         new_right_node.left.0 = left_right_node;
//         new_right_node.right.0 = right_tree;
//         // the previous root node will become right node,link the right node of previous root node and the
//         // right node of the left node of the previous root node
//         //
//         //         A                            B
//         //        / \     Right Rotation       / \
//         //       B   C    ==============>     D   A
//         //      / \                              / \
//         //     D   E                            E   C
//         //

//         self.right.0 = new_right_tree;

//         if let Some(node) = self.right.0.as_mut() {
//             node.update_height();
//         }

//         self.update_height();
//         // update height
//         true
//     }

//     pub fn rotation_left(&mut self) -> bool {
//         // The logic is same, just switch every right and left

//         if self.right.0.is_none() {
//             return false;
//         }

//         let left_node = self.right.0.as_mut().unwrap();
//         let left_right_node = left_node.left.0.take();
//         let left_left_node = left_node.right.0.take();

//         let mut new_right_tree = replace(&mut self.right.0, left_left_node);

//         swap(&mut self.value, &mut new_right_tree.as_mut().unwrap().value);

//         let right_tree = self.left.0.take();

//         let new_right_node = new_right_tree.as_mut().unwrap();
//         new_right_node.right.0 = left_right_node;
//         new_right_node.left.0 = right_tree;

//         self.left.0 = new_right_tree;

//         if let Some(node) = self.left.0.as_mut() {
//             node.update_height();
//         }

//         self.update_height();

//         true
//     }

//     pub fn rebalance(&mut self) {
//         //It is a little complex in rebalance method, because we need to consider four situations, give
//         //a node, if the balance factor is 2, then we check the left child, if the left child balance factor
//         //is 1, it is LL situation, so we just right rotate the node. If the balance factor of the left child
//         //is 0, it is the LR situation, we need to left rotate the left child, then right rotate the root node
//         //
//         //If the balance factor is -2, we check the right child, if the balance factor is -1, it is RR situation
//         //we just left rotate, if the balance factor isn't 1, it is RL situation, we need to right rotate the
//         // child and then left rotate the root node.

//         match self.balance_factor() {
//             -2 => {
//                 let right_node = self.right.0.as_mut().unwrap();

//                 if right_node.balance_factor() == 1 {
//                     right_node.rotation_right();
//                 }
//                 self.rotation_left();
//             }

//             2 => {
//                 let left_node = self.left.0.as_mut().unwrap();

//                 if left_node.balance_factor() == -1 {
//                     left_node.rotation_left();
//                 }

//                 self.rotation_right();
//             }
//             _ => {}
//         }
//     }
// }

// impl<T: Ord + Debug> AvlTree<T> {
//     pub fn tree_height(&self) -> usize {
//         match self.0 {
//             Some(ref t) => t.height,
//             None => 0,
//         }
//     }

//     pub fn set_tree_height(&mut self) {
//         if let Some(ref mut t) = self.0 {
//             t.height = 1 + max(t.left.tree_height(), t.right.tree_height());
//         }
//     }

//     pub fn new() -> Self {
//         AvlTree(None)
//         // Generate an empty AVL tree
//     }

//     pub fn print_in_order(&self, depth: usize) {
//         // I wrote an improved version of middle order traversal, which can print out the structure
//         // of the tree more intuitively. The number of '.' in front of the printed value
//         // the number of layers, and the top to the bottom represents the structure of the tree from
//         // right to left

//         if let Some(ref node) = self.0 {
//             node.right.print_in_order(depth + 1);
//             let mut spec = String::new();
//             for _ in 0..depth {
//                 spec.push('.');
//             }
//             println!("{}{:?}", spec, node.value);
//             node.left.print_in_order(depth + 1)
//         }
//     }

//     pub fn Insert(&mut self, value: T) {
//         //For insert method, we need to find the appropriate position for the value, so we need to stat
//         //from the root node 'self.0', compare the value with the current node value, if the value is less
//         //than the current node, move to the left, or move to the right, if equal, stop and rise panic, because
//         //we can not insert same element. Repeat untile we find empty node and insert the value.

//         let mut prev_ptrs = Vec::<*mut AvlNode<T>>::new();
//         //We need to traverse upwards after inserting every nodes to rebalance the tree, the popular way
//         //is to use Rc and RefCell, which allow multiple mutable reference, beacuse I want decrease the
//         //complexity of the code, I use vec to track them, which can work like a stack, I can use pop method
//         //to track the prevent node.

//         let mut current_tree = &mut self.0;

//         // root node
//         while let Some(current_node) = current_tree {
//             prev_ptrs.push(&mut **current_node);
//             // Converting mutable to a pointer, use vec to track the previous node

//             match current_node.value.cmp(&value) {
//                 Ordering::Less => current_tree = &mut current_node.right.0, //move to right
//                 Ordering::Equal => panic!("Insert same element!!"),
//                 Ordering::Greater => current_tree = &mut current_node.left.0, //move to left
//             }
//         }
//         // found an empty node, insert the value
//         *current_tree = Some(Box::new(AvlNode {
//             value,
//             left: AvlTree::new(),
//             right: AvlTree::new(),
//             height: 0,
//         }));

//         for node_pointer in prev_ptrs.into_iter().rev() {
//             let node = unsafe { &mut *node_pointer };
//             //converting a mutable pointer back to reference
//             node.update_height();
//             node.rebalance();
//         }
//     }

//     pub fn Delete(&mut self, value: &T) -> Option<T> {
//         let mut prev_pointer_vec = Vec::<*mut AvlNode<T>>::new();
//         //Track the previous node, it is very important in delete method

//         let mut current_tree = &mut self.0;
//         //Start from root node
//         let mut target_value = None;

//         while let Some(current_node) = current_tree {
//             match current_node.value.cmp(&value) {
//                 // find the node which has target value
//                 Ordering::Less => {
//                     prev_pointer_vec.push(&mut **current_node);
//                     current_tree = &mut current_node.right.0;
//                 }
//                 Ordering::Equal => {
//                     // find the node
//                     target_value = Some(&mut **current_node);
//                     break;
//                 }
//                 Ordering::Greater => {
//                     prev_pointer_vec.push(&mut **current_node);
//                     current_tree = &mut current_node.left.0;
//                 }
//             };
//         }

//         if target_value.as_ref().is_none() {
//             return None;
//         }

//         let target_node = target_value.unwrap();

//         let taken_value = if target_node.left.0.is_none() || target_node.right.0.is_none() {
//             // the node has one child or zero child
//             if let Some(left_child) = target_node.left.0.take() {
//                 replace(target_node, *left_child).value
//             }
//             //just take the child
//             else if let Some(right_child) = target_node.right.0.take() {
//                 replace(target_node, *right_child).value
//             } else if let Some(prevent_pointer) = prev_pointer_vec.pop() {
//                 // get the parent node

//                 let prevent_node = unsafe { &mut *prevent_pointer };

//                 let No_child_value = if let Some(ref left_node) = prevent_node.left.0.as_ref() {
//                     // check left or right
//                     if left_node.value == target_node.value {
//                         prevent_node.left.0.take().unwrap().value
//                     } else {
//                         prevent_node.right.0.take().unwrap().value
//                     }
//                 } else {
//                     prevent_node.right.0.take().unwrap().value
//                 };

//                 prevent_node.update_height();
//                 prevent_node.rebalance();
//                 //update and rebalance

//                 No_child_value
//             //return the value we delete
//             } else {
//                 self.0.take().unwrap().value //it is parent, just take the root
//             }
//         } else {
//             //if the directly right node has no left node, just replace the node with the right child
//             let right_tree = &mut target_node.right;

//             if right_tree.0.as_ref().unwrap().left.0.is_none() {
//                 let mut right_node = right_tree.0.take().unwrap();

//                 let value = replace(&mut target_node.value, right_node.value);
//                 replace(&mut target_node.right, right_node.right);

//                 target_node.update_height();
//                 target_node.rebalance();

//                 value
//             } else {
//                 // if the node have two children, it is a very tricky problem
//                 // First we need consider whether the child node has child node
//                 // If the right node of the root node has a left child node, we can use this node replace
//                 // the root node, which will obey the binary tree logic, if this node also has a child, we
//                 // just link it to the root node's right node.
//                 let mut right_tree = &mut target_node.right.0;
//                 let mut next_tree = right_tree;

//                 let mut ptrs = Vec::<*mut AvlNode<T>>::new();

//                 while let Some(next_left_node) = next_tree {
//                     if next_left_node.left.0.is_some() {
//                         ptrs.push(&mut **next_left_node);
//                     }
//                     next_tree = &mut next_left_node.left.0;
//                 }

//                 let parent_left_node = unsafe { &mut *ptrs.pop().unwrap() };
//                 let mut leftmost = parent_left_node.left.0.take().unwrap();

//                 let two_children_value = replace(&mut target_node.value, leftmost.value);

//                 replace(&mut parent_left_node.left, leftmost.right);

//                 parent_left_node.update_height();
//                 parent_left_node.rebalance();

//                 for ptr in ptrs.into_iter().rev() {
//                     let node = unsafe { &mut *ptr };
//                     node.update_height();
//                     node.rebalance();
//                 }

//                 target_node.update_height();
//                 target_node.rebalance();

//                 two_children_value
//             }
//         };

//         for ptr in prev_pointer_vec.into_iter().rev() {
//             let node = unsafe { &mut *ptr };
//             node.update_height();
//             node.rebalance();
//         }
//         Some(taken_value)
//     }

//     pub fn check_empty(&self) -> bool {
//         self.0.is_none()
//     }

//     pub fn Search(&self, value: &T) -> Option<&T> {
//         let mut current_tree = &self.0;

//         while let Some(current_node) = current_tree {
//             match current_node.value.cmp(&value) {
//                 Ordering::Less => {
//                     current_tree = &current_node.right.0;
//                 }
//                 Ordering::Equal => {
//                     return Some(&current_node.value);
//                 }
//                 Ordering::Greater => {
//                     current_tree = &current_node.left.0;
//                 }
//             };
//         }
//         None
//     }
// }


use std::rc::Rc;
use std::cell::RefCell;
use std::io;
use std::fs::File;
use std::io::prelude::*;

type AVLTreePtr<T> = Option<Rc<RefCell<AVLNode<T>>>>;

#[cfg(feature = "debug_print")]
macro_rules! debug_println {
    ($($args:tt)*)=>{
        println!($($args)*);
    };
}

#[cfg(not(feature = "debug_print"))]
macro_rules! debug_println {
    ($($args:tt)*)=>{};
}


#[derive(Debug, Clone)]
pub struct AVLNode<T: Ord + Clone> {
    pub value: T,
    pub left: AVLTreePtr<T>,
    pub right: AVLTreePtr<T>,
    pub height: isize,
}

#[derive(Debug)]
pub struct AVLTree<T: Ord + Clone> {
    pub root: AVLTreePtr<T>,
}

impl<T: Ord + Clone> AVLNode<T> {
    fn new(value: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(AVLNode {
            value,
            left: None,
            right: None,
            height: 1,
        }))
    }

    fn balance_factor(&self) -> isize {
        let lh = self.left.as_ref().map_or(0, |l| l.borrow().height);
        let rh = self.right.as_ref().map_or(0, |r| r.borrow().height);
        lh - rh
    }

    fn update_height(&mut self) {
        let lh = self.left.as_ref().map_or(0, |l| l.borrow().height);
        let rh = self.right.as_ref().map_or(0, |r| r.borrow().height);
        self.height = 1 + std::cmp::max(lh, rh);
    }
}

impl<T: Ord + Clone + std::fmt::Display> AVLTree<T> {

    pub fn print_tree(&self) {
        self.print_tree_rec(&self.root, 0);
    }

    fn print_tree_rec(&self, node: &AVLTreePtr<T>, level: usize) {
        if let Some(curr) = node {
            // Print right subtree with increased indentation
            self.print_tree_rec(&curr.borrow().right, level + 1);

            // Print the current node value with the current level of indentation
            for _ in 0..level {
                print!("   ");
            }
            println!("{}", curr.borrow().value);

            // Print left subtree with increased indentation
            self.print_tree_rec(&curr.borrow().left, level + 1);
        }
    }

    pub fn new() -> Self {
        AVLTree { root: None }
    }

    pub fn check_empty(&self) -> bool {
        self.root.is_none()
    }

    pub fn Insert(&mut self, value: T) {
        if self.root.is_none() {
            self.root = Some(AVLNode::new(value));
        } else {
            let taken_root = self.root.take();
            self.root = Some(self.insert_rec(taken_root, value));
        }
    }
    
    // 
    fn insert_rec(&self, node: AVLTreePtr<T>, value: T) -> Rc<RefCell<AVLNode<T>>> {
        let current_node = node.clone().unwrap_or_else(|| AVLNode::new(value.clone()));
        {
            let mut current_node_ref = current_node.borrow_mut();
            if value < current_node_ref.value {
                current_node_ref.left = Some(self.insert_rec(current_node_ref.left.clone(), value));
            } else if value > current_node_ref.value {
                current_node_ref.right = Some(self.insert_rec(current_node_ref.right.clone(), value));
            } else {
                return current_node.clone();
            }
            current_node_ref.update_height();
        }
        self.balance(current_node)
    }

    fn balance(&self, node: Rc<RefCell<AVLNode<T>>>) -> Rc<RefCell<AVLNode<T>>> {
        let balance_factor = node.borrow().balance_factor();

        // Left heavy
        if balance_factor > 1 {
            let left_child_balance = node.borrow().left.as_ref().unwrap().borrow().balance_factor();
            if left_child_balance < 0 {
                let left_child = node.borrow_mut().left.clone().unwrap();
                node.borrow_mut().left = Some(self.rotate_left(left_child));
            }
            return self.rotate_right(node);
        }

        // Right heavy
        if balance_factor < -1 {
            let right_child_balance = node.borrow().right.as_ref().unwrap().borrow().balance_factor();
            if right_child_balance > 0 {
                let right_child = node.borrow_mut().right.clone().unwrap();
                node.borrow_mut().right = Some(self.rotate_right(right_child));
            }
            return self.rotate_left(node);
        }

        node
    }

    fn rotate_left(&self, x: Rc<RefCell<AVLNode<T>>>) -> Rc<RefCell<AVLNode<T>>> {
        debug_println!("Rotating left node with value: {}", x.borrow().value);

        let y = x.borrow_mut().right.take().expect("rotate_left requires a right child");
        let t2 = y.borrow_mut().left.take();
    
        // Perform rotation
        y.borrow_mut().left = Some(x.clone());
        x.borrow_mut().right = t2;
    
        // Update heights
        x.borrow_mut().update_height();
        y.borrow_mut().update_height();
    
        y
    }
    

    fn rotate_right(&self, y: Rc<RefCell<AVLNode<T>>>) -> Rc<RefCell<AVLNode<T>>> {
        debug_println!("Rotating right node with value: {}", y.borrow().value);

        let x = y.borrow_mut().left.take().unwrap();
        let t3 = x.borrow_mut().right.take();

        // Perform rotation
        x.borrow_mut().right = Some(y.clone());
        y.borrow_mut().left = t3;

        // Update heights
        y.borrow_mut().update_height();
        x.borrow_mut().update_height();

        x
    }
    pub fn Delete(&mut self, value: T) {
        let taken_root = self.root.take();
        self.root = self.delete_rec(taken_root, value);
    }
    fn delete_rec(&self, node: AVLTreePtr<T>, value: T) -> AVLTreePtr<T> {
        if let Some(current_node) = node {
            {
                let mut node_borrow = current_node.borrow_mut();

                if value < node_borrow.value {
                    node_borrow.left = self.delete_rec(node_borrow.left.take(), value);
                } else if value > node_borrow.value {
                    node_borrow.right = self.delete_rec(node_borrow.right.take(), value);
                } else {
                    if node_borrow.left.is_some() && node_borrow.right.is_some() {
                        // Find the inorder successor's value
                        let temp = self.min_value_node(node_borrow.right.as_ref().unwrap().clone());
                        let inorder_successor_value = temp.borrow().value.clone();
                        // Now, delete the inorder successor.
                        node_borrow.right = self.delete_rec(node_borrow.right.take(), inorder_successor_value.clone());
                        // Assign the inorder successor value to the current node.
                        node_borrow.value = inorder_successor_value;
                    } else if node_borrow.left.is_some() {
                        return Some(self.balance(node_borrow.left.take().unwrap()));
                    } else if node_borrow.right.is_some() {
                        return Some(self.balance(node_borrow.right.take().unwrap()));
                    } else {
                        return None;
                    }
                }
                node_borrow.update_height();
            }
            Some(self.balance(current_node))
        } else {
            None
        }
    }
        
    fn min_value_node(&self, node: Rc<RefCell<AVLNode<T>>>) -> Rc<RefCell<AVLNode<T>>> {
        let mut current = node;
        while current.borrow().left.is_some() {
            let next_node = current.borrow().left.clone();
            if let Some(inner_node) = next_node {
                current = inner_node;
            }
        }
        current
    }
    pub fn count_leaves(&self) -> usize {
        self.count_leaves_rec(&self.root)
    }

    fn count_leaves_rec(&self, node: &AVLTreePtr<T>) -> usize {
        if let Some(curr) = node {
            let l_count = self.count_leaves_rec(&curr.borrow().left);
            let r_count = self.count_leaves_rec(&curr.borrow().right);
            if curr.borrow().left.is_none() && curr.borrow().right.is_none() {
                return 1;
            }
            l_count + r_count
        } else {
            0
        }
    }
    pub fn print_in_order(&self) -> Vec<T> {
        let mut result = Vec::new();
        self.inorder_traversal_rec(&self.root, &mut result);
        result
    }

    fn inorder_traversal_rec(&self, node: &AVLTreePtr<T>, result: &mut Vec<T>) {
        if let Some(curr) = node {
            self.inorder_traversal_rec(&curr.borrow().left, result);
            result.push(curr.borrow().value.clone());
            self.inorder_traversal_rec(&curr.borrow().right, result);
        }
    }

    pub fn tree_height(&self) -> isize {
        self.root.as_ref().map_or(0, |r| r.borrow().height)
    }
    pub fn to_dot(&self) -> String {
        let mut dot = String::from("digraph AVLTree {\n");
        self.to_dot_rec(&self.root, &mut dot);
        dot.push_str("}\n");
        dot
    }
    
    fn to_dot_rec(&self, node: &AVLTreePtr<T>, output: &mut String) {
        if let Some(curr) = node {
            let curr_val = format!("{}", curr.borrow().value);
            if let Some(left) = &curr.borrow().left {
                let left_val = format!("{}", left.borrow().value);
                output.push_str(&format!("    {} -> {} [label=\"L\"];\n", curr_val, left_val));
                self.to_dot_rec(&curr.borrow().left, output);
            }
            if let Some(right) = &curr.borrow().right {
                let right_val = format!("{}", right.borrow().value);
                output.push_str(&format!("    {} -> {} [label=\"R\"];\n", curr_val, right_val));
                self.to_dot_rec(&curr.borrow().right, output);
            }
        }
    }
    pub fn search(&self, value: T) -> bool {
        self.search_rec(&self.root, value)
    }

    fn search_rec(&self, node: &AVLTreePtr<T>, value: T) -> bool {
        match node {
            Some(current_node) => {
                let current_value = &current_node.borrow().value;
                if *current_value == value {
                    true
                } else if value < *current_value {
                    self.search_rec(&current_node.borrow().left, value)
                } else {
                    self.search_rec(&current_node.borrow().right, value)
                }
            },
            None => false,
        }
    }

}