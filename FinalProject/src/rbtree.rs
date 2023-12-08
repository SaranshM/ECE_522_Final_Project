use std::cell::RefCell;
use std::rc::Rc;
use std::mem::replace;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt;
use std::collections::VecDeque;

#[allow(non_snake_case)]

#[derive(Clone, Debug, PartialEq, Copy)]
enum NodeColor {
    Red,
    Black,
}

type TreeNode<T> = Rc<RefCell<Node<T>>>;
type Tree<T> = Option<TreeNode<T>>;

#[derive(Clone)]
pub struct Node<T: Ord+Display+Debug> {
    color: NodeColor,
    key: T,
    parent: Tree<T>,
    left: Tree<T>,
    right: Tree<T>,
}

impl<T> Node<T>
where 
    T: Debug+Ord+Display+Copy
{
    pub fn new(key: T) -> Tree<T> {
        Some(Rc::new(RefCell::new(Node {
            color: NodeColor::Red,
            key: key,
            parent: None,
            left: None,
            right: None,
        })))
    }
}

impl<T> fmt::Debug for Node<T>
where T: Debug+Ord+Display+Copy
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Node")
         .field("color", &self.color)
         .field("key", &self.key)
         .field("right", &self.right)
         .field("left", &self.left)
         .finish()
    }
}

enum Direction {
    Left,
    Right
}

#[derive(Clone, Debug)]
pub struct RBTree<T: Ord+Display+Debug+Copy> {
    root: Tree<T>,
    count: u32,
}

impl<T> RBTree<T>
where T: Ord+Display+Debug+Clone+Copy
{
    pub fn new() -> Self {
        RBTree {
            root: None,
            count: 0,
        }
    }

    pub fn count(&self) -> u32 {
        self.count
    }

    // check if tree is empty
    pub fn is_empty(&self) -> bool {
        if self.root.is_none() {
            return true;
        } else {
            return false;
        }
    }

    // insert a node to the red-black tree
    pub fn insert(&mut self, key: T) {
        if self.search(key).is_none() {
            let root = replace(&mut self.root, None);
            let updated_tree = self.insert_node(root, key);
            self.root = self.insert_fix(updated_tree.1);
        } else {
            println!("Key already in tree");
        }
    }

    fn insert_node(&mut self, tree: Tree<T>, key: T) -> (Tree<T>,TreeNode<T>) {
        match tree {
            Some(tree_node) => {
                let sub_tree: TreeNode<T>;
                let node_clone = tree_node.borrow().clone();
                if key < node_clone.key {
                    let res = self.insert_node(node_clone.left, key);
                    let res_tree = res.0;
                    sub_tree = res.1;
                    res_tree.as_ref().unwrap().borrow_mut().parent = Some(tree_node.clone());
                    tree_node.borrow_mut().left = res_tree;
                } else {
                    let res = self.insert_node(node_clone.right, key);
                    let res_tree = res.0;
                    sub_tree = res.1;
                    res_tree.as_ref().unwrap().borrow_mut().parent = Some(tree_node.clone());
                    tree_node.borrow_mut().right = res_tree;
                };
                (Some(tree_node),sub_tree)
            },
            None => {
                self.count += 1;
                let added_node = Node::<T>::new(key);
                (added_node.clone(),added_node.unwrap())
            }
        }
    }

    fn insert_fix(&mut self, tree_node: TreeNode<T>) -> Tree<T> {
        let mut is_root = tree_node.borrow().parent.is_none(); 
        let root = if is_root {
            Some(tree_node)
        } else {
            let mut node = tree_node.clone();
            let mut parent_clone = tree_node.borrow().parent.as_ref().unwrap().borrow().clone();
            let mut parent_color = parent_clone.color;
            
            while !is_root && parent_color == NodeColor::Red {
                let node_clone = node.borrow().clone();
                let uncle_return = match node_clone.parent {
                    Some(parent) => {
                        let parent = parent.borrow().clone();
                        match parent.parent {
                            Some(grandparent) => {
                                let grandparent = grandparent.borrow().clone();
                                if grandparent.key < parent.key {
                                    Some((grandparent.left.clone(), Direction::Left))
                                } else {
                                    Some((grandparent.right.clone(), Direction::Right))
                                }
                            },
                            None => {None}
                        }
                    },
                    None => { None }
                };
                
                match uncle_return {
                    Some(uncle) => {
                        let uncle_node = uncle.0;
                        let side = uncle.1;

                        match side {
                            Direction::Right => {
                                let mut parent = node.borrow().parent.as_ref().unwrap().clone();
                                if uncle_node.is_some() && uncle_node.as_ref().unwrap().borrow().color == NodeColor::Red {
                                    parent.borrow_mut().color = NodeColor::Black;
                                    uncle_node.unwrap().borrow_mut().color = NodeColor::Black;
                                    parent.borrow().parent.as_ref().unwrap().borrow_mut().color = NodeColor::Red;
                                    node = parent.borrow().clone().parent.clone().unwrap();
                                } else {
                                    if parent.borrow().clone().key < node.borrow().clone().key {
                                        let parent_tmp = node.borrow().parent.as_ref().unwrap().clone();
                                        node = parent_tmp;
                                        self.rotate_left(node.clone());
                                        parent = node.borrow().parent.as_ref().unwrap().clone();
                                    } 

                                    parent.borrow_mut().color = NodeColor::Black;
                                    parent.borrow().parent.as_ref().unwrap().borrow_mut().color = NodeColor::Red;
                                    let grandparent = node.borrow().parent.as_ref().unwrap().borrow().parent.as_ref().unwrap().clone();
                                    self.rotate_right(grandparent);
                                }
                            },
                            Direction::Left => {
                                let mut parent = node.borrow().parent.as_ref().unwrap().clone();
                                if uncle_node.is_some() && uncle_node.as_ref().unwrap().borrow().color == NodeColor::Red {
                                    parent.borrow_mut().color = NodeColor::Black;
                                    uncle_node.unwrap().borrow_mut().color = NodeColor::Black;
                                    parent.borrow().parent.as_ref().unwrap().borrow_mut().color = NodeColor::Red;
                                    node = parent.borrow().clone().parent.clone().unwrap();
                                } else {
                                    if parent.borrow().clone().key > node.borrow().clone().key {
                                        let parent_tmp = node.borrow().parent.as_ref().unwrap().clone();
                                        node = parent_tmp;
                                        self.rotate_right(node.clone());
                                        parent = node.borrow().parent.as_ref().unwrap().clone();
                                    }
                                    parent.borrow_mut().color = NodeColor::Black;
                                    parent.borrow().parent.as_ref().unwrap().borrow_mut().color = NodeColor::Red;
                                    let grandparent = node.borrow().parent.as_ref().unwrap().borrow().parent.as_ref().unwrap().clone();
                                    self.rotate_left(grandparent);
                                }
                            }
                        }
                    },
                    None => {
                        break;
                    }
                }
                is_root = node.borrow().parent.is_none();
                if !is_root {
                    parent_clone = node.borrow().parent.as_ref().unwrap().borrow().clone();
                    parent_color = parent_clone.color;
                }
            } 

            while node.borrow().parent.is_some() {
                let p = node.borrow().parent.as_ref().unwrap().clone();
                node = p;
            }
            Some(node)
        };
        root.as_ref().unwrap().borrow_mut().color = NodeColor::Black;
        root
    }

    
    fn rotate_left(&mut self, tree_node: TreeNode<T>) {
        let cur_parent = tree_node;
        let right_child = cur_parent.borrow().right.clone();
    
        cur_parent.borrow_mut().right = right_child.as_ref().and_then(|n| n.borrow().left.clone());
    
        if let Some(ref right_child) = right_child {
            if let Some(ref right_child_left) = right_child.borrow().left {
                right_child_left.borrow_mut().parent = Some(cur_parent.clone());
            }
        }
    
        if let Some(ref right_child) = right_child {
            right_child.borrow_mut().parent = cur_parent.borrow().parent.clone();
        }
    
        match cur_parent.borrow().parent.clone() {
            Some(ref grandparent) => {
                let left_child_ptr = grandparent.borrow().left.as_ref().map(|n| n.as_ptr()).unwrap_or(std::ptr::null_mut());
                let cur_parent_ptr = cur_parent.as_ptr();
                
                if std::ptr::eq(left_child_ptr, cur_parent_ptr) {
                    grandparent.borrow_mut().left = right_child.clone();
                } else {
                    grandparent.borrow_mut().right = right_child.clone();
                }
            },
            None => {
                self.root = right_child.clone();
            },
        }
    
        if let Some(ref right_child) = right_child {
            right_child.borrow_mut().left = Some(cur_parent.clone());
        }
        cur_parent.borrow_mut().parent = right_child;
    }
    
    

    fn rotate_right(&self, tree_node: TreeNode<T>) {
        let cur_parent = tree_node;
        let left_child = cur_parent.borrow().left.clone();

        cur_parent.borrow_mut().left = match left_child {
            Some(ref left_child) => {left_child.borrow().right.clone()},
            None => {None}
        };

        if left_child.is_some() {
            left_child.as_ref().unwrap().borrow_mut().parent = cur_parent.borrow().parent.clone();
            if left_child.as_ref().unwrap().borrow().right.is_some() {
                let r = left_child.as_ref().unwrap().borrow().right.clone();
                r.unwrap().borrow_mut().parent = Some(cur_parent.clone());
            }
        }

        match cur_parent.borrow().clone().parent {
            Some(grandparent) => {
                if grandparent.borrow().clone().key < cur_parent.borrow().clone().key {
                    grandparent.borrow_mut().right = left_child.clone();
                } else {
                    grandparent.borrow_mut().left = left_child.clone();
                }
            },
            None => {
                left_child.as_ref().unwrap().borrow_mut().parent = None;
            },
        }
        left_child.as_ref().unwrap().borrow_mut().right = Some(cur_parent.clone());
        cur_parent.borrow_mut().parent = left_child.clone();
    }

    pub fn search(&self, key: T) -> Tree<T> {
        let dummy = Node::<T>::new(key).unwrap().borrow().clone();
        self.search_node(&self.root, &dummy)
    }

    fn search_node(&self, tree_node: &Tree<T>, node: &Node<T>) -> Tree<T> {
        match tree_node {
            Some(sub_tree) => {
                let sub_tree_clone = sub_tree.borrow().clone();
                if sub_tree_clone.key == node.key {
                    Some(sub_tree.clone())
                } else {
                    if sub_tree_clone.key > node.key {
                        self.search_node(&sub_tree_clone.left, node)
                    } else {
                        self.search_node(&sub_tree_clone.right, node)
                    }
                }
            },
            None => {None}
        }
    }

    pub fn search_element(&self, key: T) -> bool {
        self.search_element_rec(&self.root, key)
    }

    // Helper function to recursively search for a node
    fn search_element_rec(&self, tree_node: &Tree<T>, key: T) -> bool {
        match tree_node {
            Some(node) => {
                let node_ref = node.borrow();
                if node_ref.key == key {
                    true
                } else if key < node_ref.key {
                    self.search_element_rec(&node_ref.left, key)
                } else {
                    self.search_element_rec(&node_ref.right, key)
                }
            },
            None => false,
        }
    }

    // delete a node from the red-black tree
    pub fn delete(&mut self, key: T) {
        let z = self.search(key);
        if z.is_none() {
            println!("Key not found");
            return;
        }
        let u = z; 
        let p = u.as_ref().unwrap().borrow().parent.clone();
        let v = u.as_ref().unwrap().borrow().left.clone(); 
        let w = u.as_ref().unwrap().borrow().right.clone();

        let mut side = Direction::Left; 

        if p.is_some() {
            side = if p.as_ref().unwrap().borrow().clone().key > u.as_ref().unwrap().borrow().clone().key {
                Direction::Right
            } else {
                Direction::Left
            };
        }

        let mut u_original_color = u.as_ref().unwrap().borrow().color.clone();
        let x: Tree<T>;

        if v.is_none() {
            x = w.clone();
            self.transplant(u.clone(),w.clone());
            self.print_tree();
        } else if w.is_none() {
            x = v.clone();
            self.transplant(u.clone(), v.clone());
        } else {
            let y = self.find_min(w.clone());
            u_original_color = y.as_ref().unwrap().borrow().color.clone();
            x = y.as_ref().unwrap().borrow().clone().right;
            if y.as_ref().unwrap().borrow().clone().parent.as_ref().unwrap().borrow().clone().key == u.as_ref().unwrap().borrow().clone().key {
                if x.is_some() {
                    x.as_ref().unwrap().borrow_mut().parent = y.clone();
                }
            } else {
                self.transplant(y.clone(), y.as_ref().unwrap().borrow().right.clone());
                y.as_ref().unwrap().borrow_mut().right = u.as_ref().unwrap().borrow().right.clone();
                y.as_ref().unwrap().borrow().right.as_ref().unwrap().borrow_mut().parent = y.clone();
            }
            self.transplant(u.clone(), y.clone());
            y.as_ref().unwrap().borrow_mut().left = v.clone();
            v.as_ref().unwrap().borrow_mut().parent = y.clone();
            y.as_ref().unwrap().borrow_mut().color = u.as_ref().unwrap().borrow().color.clone();
        }
        if u_original_color == NodeColor::Black {
            self.delete_fix(x.clone(), p.clone(), side);
        }
        self.count -= 1;
    }

    fn delete_fix(&mut self, x: Tree<T>, p: Tree<T>, side: Direction) {
        self.print_tree();
        let mut x_color = if x.is_some() {
            x.as_ref().unwrap().borrow().clone().color == NodeColor::Black
        } else {
            true
        };
        let mut cur_p = p;
        let mut cur_x = x;
        let mut is_root = cur_p.is_none();
        while !is_root && x_color {
            match side {
                Direction::Right => {
                    self.print_tree();
                    let mut s = cur_p.as_ref().unwrap().borrow().right.clone();
                    if s.is_some() {
                        self.print_tree();
                        if s.as_ref().unwrap().borrow().clone().color == NodeColor::Red {
                            s.as_ref().unwrap().borrow_mut().color = NodeColor::Black;
                            cur_p.as_ref().unwrap().borrow_mut().color = NodeColor::Red;
                            self.rotate_left(cur_p.as_ref().unwrap().clone());
                            s = cur_p.as_ref().unwrap().borrow().right.clone();
                        }
                        let s_left = s.as_ref().unwrap().borrow().clone().left.clone();
                        let s_right = s.as_ref().unwrap().borrow().clone().right.clone();

                        let s_left_color = if s_left.is_some() {
                            s_left.as_ref().unwrap().borrow().clone().color == NodeColor::Black
                        } else {
                            true
                        };

                        let s_right_color = if s_right.is_some() {
                            s_right.as_ref().unwrap().borrow().clone().color == NodeColor::Black
                        } else {
                            true
                        };

                        if s_left_color && s_right_color {
                            s.as_ref().unwrap().borrow_mut().color = NodeColor::Red;
                            cur_x = cur_p.clone();
                            let g = cur_p.as_ref().unwrap().borrow().clone().parent.clone();
                            cur_p = g.clone();
                            x_color = if cur_x.is_some() {
                                cur_x.as_ref().unwrap().borrow().clone().color == NodeColor::Black
                            } else {
                                true
                            };
                        } else {
                            self.print_tree();
                            if s_right.is_some() && s_right.as_ref().unwrap().borrow().clone().color == NodeColor::Black {
                                if s_left.is_some() {
                                    s_left.as_ref().unwrap().borrow_mut().color = NodeColor::Black;
                                    s.as_ref().unwrap().borrow_mut().color = NodeColor::Red;
                                    self.rotate_right(s.unwrap());
                                    s = cur_p.as_ref().unwrap().borrow().right.clone();
                                }
                            }
                            s.as_ref().unwrap().borrow_mut().color = cur_p.as_ref().unwrap().borrow().color.clone();
                            cur_p.as_ref().unwrap().borrow_mut().color = NodeColor::Black;
                            if s_right.is_some() {
                                self.print_tree();
                                s_right.as_ref().unwrap().borrow_mut().color = NodeColor::Black;
                            }
                            self.rotate_left(cur_p.as_ref().unwrap().clone());
                            is_root = true;
                            self.print_tree();
                        }
                    }
                },
                Direction::Left => {
                    let mut s = cur_p.as_ref().unwrap().borrow().left.clone();
                    if s.is_some() {
                        if s.as_ref().unwrap().borrow().clone().color == NodeColor::Red {
                            s.as_ref().unwrap().borrow_mut().color = NodeColor::Black;
                            cur_p.as_ref().unwrap().borrow_mut().color = NodeColor::Red;
                            self.rotate_right(cur_p.as_ref().unwrap().clone());
                            s = cur_p.as_ref().unwrap().borrow().left.clone();
                        }
                        let s_left = s.as_ref().unwrap().borrow().clone().left.clone();
                        let s_right = s.as_ref().unwrap().borrow().clone().right.clone();

                        let s_left_color = if s_left.is_some() {
                            s_left.as_ref().unwrap().borrow().clone().color == NodeColor::Black
                        } else {
                            true
                        };

                        let s_right_color = if s_right.is_some() {
                            s_right.as_ref().unwrap().borrow().clone().color == NodeColor::Black
                        } else {
                            true
                        };

                        if s_left_color && s_right_color {
                            s.as_ref().unwrap().borrow_mut().color = NodeColor::Red;
                            cur_x = cur_p.clone();
                            let g = cur_p.as_ref().unwrap().borrow().clone().parent.clone();
                            cur_p = g.clone();
                            x_color = if cur_x.is_some() {
                                cur_x.as_ref().unwrap().borrow().clone().color == NodeColor::Black
                            } else {
                                true
                            };
                        } else {
                            if s_right.is_some() && s_right.as_ref().unwrap().borrow().clone().color == NodeColor::Black {
                                if s_left.is_some() {
                                    s_left.as_ref().unwrap().borrow_mut().color = NodeColor::Black;
                                    s.as_ref().unwrap().borrow_mut().color = NodeColor::Red;
                                    self.rotate_left(s.unwrap());
                                    s = cur_p.as_ref().unwrap().borrow().left.clone();
                                }
                            }
                            s.as_ref().unwrap().borrow_mut().color = cur_p.as_ref().unwrap().borrow().color.clone();
                            cur_p.as_ref().unwrap().borrow_mut().color = NodeColor::Black;
                            if s_left.is_some() {
                                s_left.as_ref().unwrap().borrow_mut().color = NodeColor::Black;
                            }
                            self.rotate_right(cur_p.as_ref().unwrap().clone());
                            is_root = true;
                        }
                    }
                }
            }
        }
        if cur_x.is_some() {
            cur_x.as_ref().unwrap().borrow_mut().color = NodeColor::Black;
        }
    }
    

    fn transplant(&mut self, z: Tree<T>, v: Tree<T>) {
        let u = z.unwrap();
        let u_p = u.borrow().parent.clone();
        if u_p.is_none() {
            self.root = v.clone();
        } else {
            if u_p.as_ref().unwrap().borrow().clone().key > u.borrow().clone().key {
                u_p.as_ref().unwrap().borrow_mut().left = v.clone();
            } else {
                u_p.as_ref().unwrap().borrow_mut().right = v.clone();
            }
        }
        if v.is_some() {
            v.as_ref().unwrap().borrow_mut().parent = u_p.clone();
        }
    }

    fn find_min(&self, tree: Tree<T>) -> Tree<T> {
        match tree {
            Some(sub_tree) => {
                let mut left = Some(sub_tree.clone());
                while left.as_ref().unwrap().borrow().left.clone().is_some() {
                    left = left.unwrap().borrow().left.clone();
                }
                left
            },
            None => {
                tree
            }
        }
    }

    fn find_max(&self, tree: Tree<T>) -> Tree<T> {
        match tree {
            Some(sub_tree) => {
                let mut right = Some(sub_tree.clone());
                while right.as_ref().unwrap().borrow().right.clone().is_some() {
                    right = right.unwrap().borrow().right.clone();
                }
                right
            },
            None => {
                tree
            }
        }
    }

    // count the number of leaves in a tree
    pub fn leaves(&self) -> u32 {
        if self.root.is_none() {
            return 0;
        }
        let root = self.root.as_ref().unwrap().clone();
        let mut stack: Vec<Tree<T>> = Vec::new();
        stack.push(Some(root));

        let mut count = 0;
        while !stack.is_empty() {
            let node = stack.pop();
            let mut node_left = None;
            let mut node_right = None;

            if node.is_some() {
                node_left = node.as_ref().unwrap().as_ref().unwrap().borrow().clone().left.clone();
                node_right = node.as_ref().unwrap().as_ref().unwrap().borrow().clone().right.clone();
            }

            if node_left.is_some() {
                stack.push(node_left.clone());
            }

            if node_right.is_some() {
                stack.push(node_right.clone());
            }

            if node_left.is_none() && node_right.is_none() {
                count += 1;
            }
        }
        count
    }

    // 4- return the height of a tree
    pub fn height(&self) -> u32 {
        if self.root.is_none() {
            return 0;
        }
        let root = self.root.as_ref().unwrap().clone();
        let mut queue: VecDeque<Tree<T>> = VecDeque::new();
        queue.push_back(Some(root));

        let mut height = 0;
        while !queue.is_empty() {
            let n = queue.len();
            for _ in 0..n {
                let node = queue.pop_front().unwrap().unwrap();
                for child in [node.borrow().left.clone(), node.borrow().right.clone()] {
                    if child.is_some() {
                        queue.push_back(child);
                    }
                }
            }
            height += 1;
        }
        height
    }

    // print in-order traversal of tree
    pub fn print_inorder(&self) {
        if self.root.is_none() {
            println!("None");
            return;
        }
        let mut root = self.root.clone();
        let mut stack: Vec<Tree<T>> = Vec::new();
        while !stack.is_empty() || !root.is_none() {
            if root.is_some() {
                stack.push(root.clone());
                let p = root.as_ref().unwrap().borrow().left.clone();
                root = p.clone();
            } else {
                let pop = stack.pop().unwrap();
                print!(" {} ", pop.as_ref().unwrap().borrow().key.clone());
                root = pop.as_ref().unwrap().borrow().right.clone();
            }
        }
        println!("\n");
    }

    pub fn print_preorder(&self) {
        if self.root.is_none() {
            println!("None");
            return;
        }
        let mut root = self.root.clone();
        let mut stack: Vec<Tree<T>> = Vec::new();
        stack.push(root);
        let mut cur: Tree<T>;
        while !stack.is_empty() {
            cur = stack.pop().unwrap();
            root = cur.clone();
            print!(" {} ", root.as_ref().unwrap().borrow().key.clone());
            let root_right = root.as_ref().unwrap().borrow().right.clone();
            let root_left = root.as_ref().unwrap().borrow().left.clone();
            if root_right.is_some() {
                stack.push(root_right.clone());
            }
            if root_left.is_some() {
                stack.push(root_left.clone());
            }
        }
        println!("\n");
    }

    pub fn print_levelorder(&self) {
        if self.root.is_none() {
            println!("None");
            return;
        };
        let inorder_nodes = self.inorder();
        for node in inorder_nodes {
            print!(" {} ", node.unwrap().borrow().key.clone());
        }
        println!("\n");
    }

    pub fn min(&self) -> Tree<T> {
        self.find_min(self.root.clone())
    }

    pub fn max(&self) -> Tree<T> {
        self.find_max(self.root.clone())
    }

    fn inorder(&self) -> VecDeque<Tree<T>> {
        let root = self.root.as_ref().unwrap().clone();
        let mut queue: VecDeque<Tree<T>> = VecDeque::new();
        queue.push_back(Some(root));
        let mut res: VecDeque<Tree<T>> = VecDeque::new();
        while !queue.is_empty() {
            let n = queue.len();
            for _ in 0..n {
                let node = queue.pop_front().unwrap().unwrap();
                res.push_back(Some(node.clone()));
                for child in [node.borrow().left.clone(), node.borrow().right.clone()] {
                    if child.is_some() {
                        queue.push_back(child);
                    }
                }
            }
        }
        res
    }

    pub fn print_tree(&self) {
        self.print_node(&self.root, 0, "Root", "");
    }

    fn print_node(&self, node: &Tree<T>, depth: usize, node_type: &str, prefix: &str) {
        if let Some(ref node) = node {
            let color = match node.borrow().color {
                NodeColor::Black => "black",
                NodeColor::Red => "red",
            };

            let new_prefix = if node_type == "Root" {
                "── "
            } else {
                if node_type == "L" { "├── " } else { "└── " }
            };

            println!("{}{}[{}] {}: {}", prefix, new_prefix, color, node_type, node.borrow().key);

            let child_prefix = if node_type == "L" {
                format!("{}│   ", prefix)
            } else {
                format!("{}    ", prefix)
            };

            self.print_node(&node.borrow().left, depth + 1, "L", &child_prefix);
            self.print_node(&node.borrow().right, depth + 1, "R", &child_prefix);
        }
    }
}

impl<T> fmt::Display for RBTree<T>
where T: Debug+Ord+Display+Copy
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("RBTree")
         .field("root", &self.root)
         .field("length", &self.count)
         .finish()
    }
}