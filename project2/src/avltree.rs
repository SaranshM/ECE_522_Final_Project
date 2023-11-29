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

    pub fn is_empty(&self) -> bool {
        self.root.is_none()
    }

    pub fn insert(&mut self, value: T) {
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
    pub fn delete(&mut self, value: T) {
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
    pub fn inorder_traversal(&self) -> Vec<T> {
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

    pub fn height(&self) -> isize {
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

    pub fn print_preorder(&self) -> Vec<T> {
        // @abhishek
        let mut result = Vec::new();
        self.inorder_traversal_rec(&self.root, &mut result);
        result
    }

    pub fn print_levelorder(&self) -> Vec<T> {
        // @abhishek
        let mut result = Vec::new();
        self.inorder_traversal_rec(&self.root, &mut result);
        result
    }

    pub fn count(&self) -> u32 {
        // self.count
        // @abhishek
        let x: u32 = 2; // dummy
        x
    }

}

// fn main() {
//     let mut avl_tree = AVLTree::<i32>::new();

//     loop {
//         // Display the menu
//         println!("------------------------------------------");
//         println!("AVL Tree Operations");
//         println!("------------------------------------------");
//         println!("1. Insert an element");
//         println!("2. Delete an element");
//         println!("3. Display tree in-order");
//         println!("4. Count leaves");
//         println!("5. Get tree height");
//         println!("6. Check if tree is empty");
//         println!("7. Exit");
//         println!("8. Print tree structure");
//         println!("9. Print DOT representation");
//         println!("10. Search for an element");
//         println!("------------------------------------------");

//         // Read user's choice
//         let mut choice = String::new();
//         io::stdin().read_line(&mut choice).expect("Failed to read line");

//         match choice.trim().parse::<u32>() {
//             Ok(1) => {
//                 println!("Enter the element to insert:");
//                 let mut element = String::new();
//                 io::stdin().read_line(&mut element).expect("Failed to read line");
//                 let element: i32 = element.trim().parse().expect("Please enter a valid number");
//                 avl_tree.insert(element);
//                 println!("Element inserted successfully!");
//             },
//             Ok(2) => {
//                 println!("Enter the element to delete:");
//                 let mut element = String::new();
//                 io::stdin().read_line(&mut element).expect("Failed to read line");
//                 let element: i32 = element.trim().parse().expect("Please enter a valid number");
//                 avl_tree.delete(element);
//                 println!("Element deleted successfully!");
//             },
//             Ok(3) => {
//                 println!("In-order Traversal: {:?}", avl_tree.inorder_traversal());
//             },
//             Ok(4) => {
//                 println!("Leaves Count: {}", avl_tree.count_leaves());
//             },
//             Ok(5) => {
//                 println!("Tree Height: {}", avl_tree.height());
//             },
//             Ok(6) => {
//                 if avl_tree.is_empty() {
//                     println!("The AVL tree is empty.");
//                 } else {
//                     println!("The AVL tree is not empty.");
//                 }
//             },
//             Ok(7) => {
//                 println!("Thank you for using the AVL Tree CLI. Goodbye!");
//                 break;
//             },
//             Ok(8) => {
//                 println!("Tree Structure");
//                 avl_tree.print_tree();
//             },
//             Ok(9) => {
//                 let dot_representation = avl_tree.to_dot();
//                 println!("{}", dot_representation);
                
//                 // Save to a file
//                 let mut file = File::create("output.dot").expect("Could not create file");
//                 file.write_all(dot_representation.as_bytes()).expect("Could not write to file");
//                 println!("DOT representation saved to output.dot");
//             },
//             Ok(10) => {
//                 println!("Enter the element to search:");
//                 let mut element = String::new();
//                 io::stdin().read_line(&mut element).expect("Failed to read line");
//                 let element: i32 = element.trim().parse().expect("Please enter a valid number");

//                 let found = avl_tree.search(element);
//                 if found {
//                     println!("Element {} found in the AVL tree.", element);
//                 } else {
//                     println!("Element {} not found in the AVL tree.", element);
//                 }
//             },
//             _ => {
//                 println!("Invalid choice! Please select a valid option from the menu.");
//             }
//         }
//     }
// }