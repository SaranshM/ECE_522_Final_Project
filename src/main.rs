use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Clone, Debug, PartialEq)]
enum NodeColor {
    Red,
    Black,
}

type Tree<T> = Rc<RefCell<TreeNode<T>>>;
type RedBlackTree<T> = Option<Tree<T>>;
type WeakLink<T> = Option<Weak<RefCell<TreeNode<T>>>>;

#[derive(Debug)]
struct TreeNode<T: Ord> {
    color: NodeColor,
    key: T,
    parent: WeakLink<T>,
    left: RedBlackTree<T>,
    right: RedBlackTree<T>,
}

impl<T: Ord> TreeNode<T> {
    fn new(key: T) -> Self {
        TreeNode {
            color: NodeColor::Red,
            key,
            parent: None,
            left: None,
            right: None,
        }
    }
}


#[derive(Debug)]
struct RedBlackTreeRoot<T: Ord> {
    root: RedBlackTree<T>,
}

impl<T: Ord + std::fmt::Debug + Clone> RedBlackTreeRoot<T> {
    pub fn new() -> Self {
        RedBlackTreeRoot { root: None }
    }

    pub fn insert(&mut self, key: T) {
        let new_node = Rc::new(RefCell::new(TreeNode::new(key)));
        if self.root.is_some() {
            self.insert_recursive(self.root.clone(), &new_node); 
        } else {
            self.root = Some(new_node.clone());
        }
        self.root.as_ref().unwrap().borrow_mut().color = NodeColor::Black; 
        self.fix_violations(Some(new_node));
    }

    fn insert_recursive(&self, current: RedBlackTree<T>, new_node: &Tree<T>) {
        if let Some(curr_node) = current {
            if &new_node.borrow().key < &curr_node.borrow().key {
                if curr_node.borrow().left.is_none() {
                    new_node.borrow_mut().parent = Some(Rc::downgrade(&curr_node));
                    curr_node.borrow_mut().left = Some(new_node.clone());
                } else {
                    self.insert_recursive(curr_node.borrow().left.clone(), new_node);
                }
            } else if &new_node.borrow().key > &curr_node.borrow().key {
                if curr_node.borrow().right.is_none() {
                    new_node.borrow_mut().parent = Some(Rc::downgrade(&curr_node));
                    curr_node.borrow_mut().right = Some(new_node.clone());
                } else {
                    self.insert_recursive(curr_node.borrow().right.clone(), new_node);
                }
            }
        }
    }

    fn fix_violations(&mut self, mut current_node: RedBlackTree<T>) {
        while let Some(current) = current_node.clone() {
            let parent_opt = current.borrow().parent.as_ref().and_then(|p| p.upgrade());
            if parent_opt.is_none() || parent_opt.as_ref().unwrap().borrow().color == NodeColor::Black {
                break;
            }
    
            let parent_rc = parent_opt.unwrap();
            let grand_parent_opt = parent_rc.borrow().parent.as_ref().and_then(|p| p.upgrade());
            let grand_parent_rc = match grand_parent_opt {
                Some(gp) => gp,
                None => break,
            };
            let uncle = if parent_rc.borrow().left.as_ref().map_or(false, |x| Rc::ptr_eq(x, &current)) {
                grand_parent_rc.borrow().right.clone()
            } else {
                grand_parent_rc.borrow().left.clone()
            };
    
            if uncle.as_ref().map_or(false, |uncle| uncle.borrow().color == NodeColor::Red) {
                parent_rc.borrow_mut().color = NodeColor::Black;
                uncle.as_ref().unwrap().borrow_mut().color = NodeColor::Black;
                grand_parent_rc.borrow_mut().color = NodeColor::Red;
                current_node = Some(grand_parent_rc);
                continue;
            }
    
            let is_left_child = parent_rc.borrow().left.as_ref().map_or(false, |x| Rc::ptr_eq(x, &current));
            let parent_is_left_child = grand_parent_rc.borrow().left.as_ref().map_or(false, |x| Rc::ptr_eq(x, &parent_rc));
    
            match (is_left_child, parent_is_left_child) {
                (true, false) => {
                    self.rotate_left(parent_rc.clone());
                    self.rotate_right(grand_parent_rc.clone());
                    Self::swap_colors::<T>(&parent_rc, &grand_parent_rc);
                    current_node = grand_parent_rc.borrow().right.clone();
                }
                (false, true) => {
                    self.rotate_right(parent_rc.clone());
                    self.rotate_left(grand_parent_rc.clone());
                    Self::swap_colors::<T>(&parent_rc, &grand_parent_rc);
                    current_node = grand_parent_rc.borrow().left.clone();
                }
                (true, true) => {
                    self.rotate_right(grand_parent_rc.clone());
                    Self::swap_colors::<T>(&parent_rc, &grand_parent_rc);
                    current_node = None;
                }
                (false, false) => {
                    self.rotate_left(grand_parent_rc.clone());
                    Self::swap_colors::<T>(&parent_rc, &grand_parent_rc);
                    current_node = None;
                }
            }
        }
        self.root.as_ref().unwrap().borrow_mut().color = NodeColor::Black;
    }
    
    // Helper function to swap colors
    fn swap_colors<P: Ord>(a: &Tree<P>, b: &Tree<P>) {
        let a_color = a.borrow().color.clone();
        let b_color = b.borrow().color.clone();
        a.borrow_mut().color = b_color;
        b.borrow_mut().color = a_color;
    }
    
    

    fn rotate_left(&mut self, node: Tree<T>) {
        let right_child = node.borrow_mut().right.take();
        if let Some(right) = right_child {
            let right_left_child = right.borrow_mut().left.take();
            node.borrow_mut().right = right_left_child.clone();
    
            if let Some(child) = right_left_child {
                child.borrow_mut().parent = Some(Rc::downgrade(&node));
            }
    
            let parent_weak = node.borrow().parent.clone();
            right.borrow_mut().parent = parent_weak.clone();
            if let Some(parent) = parent_weak.and_then(|w| w.upgrade()) {
                if parent.borrow().left.as_ref().map_or(false, |x| Rc::ptr_eq(x, &node)) {
                    parent.borrow_mut().left = Some(right.clone());
                } else {
                    parent.borrow_mut().right = Some(right.clone());
                }
            } else {
                self.root = Some(right.clone());
            }
    
            node.borrow_mut().parent = Some(Rc::downgrade(&right));
            right.borrow_mut().left = Some(node);
        }
    }
    
    fn rotate_right(&mut self, node: Tree<T>) {
        let left_child = node.borrow_mut().left.take();
        if let Some(left) = left_child {
            let left_right_child = left.borrow_mut().right.take();
            node.borrow_mut().left = left_right_child.clone();
    
            if let Some(child) = left_right_child {
                child.borrow_mut().parent = Some(Rc::downgrade(&node));
            }
    
            let parent_weak = node.borrow().parent.clone();
            left.borrow_mut().parent = parent_weak.clone();
            if let Some(parent) = parent_weak.and_then(|w| w.upgrade()) {
                if parent.borrow().left.as_ref().map_or(false, |x| Rc::ptr_eq(x, &node)) {
                    parent.borrow_mut().left = Some(left.clone());
                } else {
                    parent.borrow_mut().right = Some(left.clone());
                }
            } else {
                self.root = Some(left.clone());
            }
    
            node.borrow_mut().parent = Some(Rc::downgrade(&left));
            left.borrow_mut().right = Some(node);
        }
    }

    pub fn delete(&mut self, key: T) {
        self.root = self.delete_recursive(self.root.clone(), key);
    }

    fn delete_recursive(&mut self, current: RedBlackTree<T>, key: T) -> RedBlackTree<T> {
        if current.is_none() {
            return current;
        }
        
        let curr_key;
        let mut left = None;
        let mut right = None;
        {
            let curr_node = current.as_ref().unwrap().borrow();
            curr_key = curr_node.key.clone();
            left = curr_node.left.clone();
            right = curr_node.right.clone();
        }
        
        if key < curr_key {
            let new_left = self.delete_recursive(left, key);
            current.as_ref().unwrap().borrow_mut().left = new_left;
        } else if key > curr_key {
            let new_right = self.delete_recursive(right, key);
            current.as_ref().unwrap().borrow_mut().right = new_right;
        } else {
            if left.is_some() && right.is_some() {
                let successor = self.min_value_node(right.clone());
                {
                    let mut curr_node_mut = current.as_ref().unwrap().borrow_mut();
                    curr_node_mut.key = successor.borrow().key.clone();
                }
                let new_right = self.delete_recursive(right, successor.borrow().key.clone());
                current.as_ref().unwrap().borrow_mut().right = new_right;
            } else {
                return left.or(right);
            }
        }
    
        current
    }
    
    fn min_value_node(&self, mut node: RedBlackTree<T>) -> Tree<T> {
        while let Some(curr_node) = node.clone() {
            if curr_node.borrow().left.is_none() {
                break;
            }
            node = curr_node.borrow().left.clone();
        }
        node.unwrap()
    }

    fn fix_delete_violations(&mut self, mut node: RedBlackTree<T>) {
        while node.as_ref().map_or(false, |x| !Rc::ptr_eq(x, &self.root.clone().unwrap())) && node.as_ref().map_or(true, |x| x.borrow().color == NodeColor::Black) {
            if self.is_left_child(node.clone()) {
                let sibling = node.as_ref().unwrap().borrow().parent.as_ref().unwrap().upgrade().unwrap().borrow().right.clone();
                if sibling.as_ref().map_or(false, |s| s.borrow().color == NodeColor::Red) {
                    sibling.as_ref().unwrap().borrow_mut().color = NodeColor::Black;
                    node.as_ref().unwrap().borrow().parent.as_ref().unwrap().upgrade().unwrap().borrow_mut().color = NodeColor::Red;
                    self.rotate_left(node.as_ref().unwrap().borrow().parent.as_ref().unwrap().upgrade().unwrap().clone());
                } else if (sibling.as_ref().map_or(true, |s| s.borrow().left.is_none() || s.borrow().left.as_ref().unwrap().borrow().color == NodeColor::Black) &&
                    sibling.as_ref().map_or(true, |s| s.borrow().right.is_none() || s.borrow().right.as_ref().unwrap().borrow().color == NodeColor::Black)) {
                    sibling.as_ref().unwrap().borrow_mut().color = NodeColor::Red;
                    let parent = node.as_ref().unwrap().borrow().parent.as_ref().unwrap().upgrade();
                    node = parent;
                } else {
                    if sibling.as_ref().map_or(true, |s| s.borrow().right.is_none() || s.borrow().right.as_ref().unwrap().borrow().color == NodeColor::Black) {
                        if let Some(left_child) = sibling.as_ref().unwrap().borrow().left.clone() {
                            left_child.borrow_mut().color = NodeColor::Black;
                        }
                        sibling.as_ref().unwrap().borrow_mut().color = NodeColor::Red;
                        self.rotate_right(sibling.clone().unwrap());
                    }
                    sibling.as_ref().unwrap().borrow_mut().color = node.as_ref().unwrap().borrow().parent.as_ref().unwrap().upgrade().unwrap().borrow().color.clone();
                    node.as_ref().unwrap().borrow().parent.as_ref().unwrap().upgrade().unwrap().borrow_mut().color = NodeColor::Black;
                    if sibling.as_ref().unwrap().borrow().right.is_some() {
                        sibling.as_ref().unwrap().borrow().right.as_ref().unwrap().borrow_mut().color = NodeColor::Black;
                    }
                    self.rotate_left(node.as_ref().unwrap().borrow().parent.as_ref().unwrap().upgrade().unwrap().clone());
                    node = self.root.clone();
                }
            } else {
                let sibling = node.as_ref().unwrap().borrow().parent.as_ref().unwrap().upgrade().unwrap().borrow().left.clone();
                if sibling.as_ref().map_or(false, |s| s.borrow().color == NodeColor::Red) {
                    sibling.as_ref().unwrap().borrow_mut().color = NodeColor::Black;
                    node.as_ref().unwrap().borrow().parent.as_ref().unwrap().upgrade().unwrap().borrow_mut().color = NodeColor::Red;
                    self.rotate_right(node.as_ref().unwrap().borrow().parent.as_ref().unwrap().upgrade().unwrap().clone());
                } else if (sibling.as_ref().map_or(true, |s| s.borrow().right.is_none() || s.borrow().right.as_ref().unwrap().borrow().color == NodeColor::Black) &&
                    sibling.as_ref().map_or(true, |s| s.borrow().left.is_none() || s.borrow().left.as_ref().unwrap().borrow().color == NodeColor::Black)) {
                    sibling.as_ref().unwrap().borrow_mut().color = NodeColor::Red;
                    let parent = node.as_ref().unwrap().borrow().parent.as_ref().unwrap().upgrade();
                    node = parent;
                } else {
                    if sibling.as_ref().map_or(true, |s| s.borrow().left.is_none() || s.borrow().left.as_ref().unwrap().borrow().color == NodeColor::Black) {
                        if let Some(right_child) = sibling.as_ref().unwrap().borrow().right.clone() {
                            right_child.borrow_mut().color = NodeColor::Black;
                        }
                        sibling.as_ref().unwrap().borrow_mut().color = NodeColor::Red;
                        self.rotate_left(sibling.clone().unwrap());
                    }
                    sibling.as_ref().unwrap().borrow_mut().color = node.as_ref().unwrap().borrow().parent.as_ref().unwrap().upgrade().unwrap().borrow().color.clone();
                    node.as_ref().unwrap().borrow().parent.as_ref().unwrap().upgrade().unwrap().borrow_mut().color = NodeColor::Black;
                    if sibling.as_ref().unwrap().borrow().left.is_some() {
                        sibling.as_ref().unwrap().borrow().left.as_ref().unwrap().borrow_mut().color = NodeColor::Black;
                    }
                    self.rotate_right(node.as_ref().unwrap().borrow().parent.as_ref().unwrap().upgrade().unwrap().clone());
                    node = self.root.clone();
                }
            }
        }

        if node.is_some() {
            node.unwrap().borrow_mut().color = NodeColor::Black;
        }
    }

    fn is_left_child(&self, node: RedBlackTree<T>) -> bool {
        let parent = node.as_ref().unwrap().borrow().parent.as_ref().and_then(|w| w.upgrade());
        parent.map_or(false, |p| p.borrow().left.as_ref().map_or(false, |x| Rc::ptr_eq(x, &node.unwrap())))
    }

    pub fn count_leaves(&self) -> usize {
        self._count_leaves(&self.root)
    }

    fn _count_leaves(&self, node: &RedBlackTree<T>) -> usize {
        if let Some(current) = node {
            let left = &current.borrow().left;
            let right = &current.borrow().right;
    
            // Check if the node is a leaf (no children)
            if left.is_none() && right.is_none() {
                return 1;
            }
    
            // Otherwise, recurse on the children and sum the results
            return self._count_leaves(left) + self._count_leaves(right);
        }
    
        0
    }

    pub fn height(&self) -> usize {
        self._height(&self.root)
    }

    fn _height(&self, node: &RedBlackTree<T>) -> usize {
        if let Some(current) = node {
            let left_height = self._height(&current.borrow().left);
            let right_height = self._height(&current.borrow().right);
    
            // The height of the node is 1 (for the current node) plus the maximum of the heights of its left and right children.
            return 1 + std::cmp::max(left_height, right_height);
        }
    
        0 // Base case: if the node is None, its height is 0.
    }
    
    

    pub fn print_custom_format(&self) {
        self._print_custom_format(&self.root, "", "R");
    }

    fn _print_custom_format(&self, node: &RedBlackTree<T>, prefix: &str, direction: &str) {
        if let Some(real_node) = node {
            println!("{}{}---- {:?}({:?})", prefix, direction, real_node.borrow().key, real_node.borrow().color);

            if real_node.borrow().left.is_some() {
                self._print_custom_format(&real_node.borrow().left, &format!("{}|     ", prefix), "L");
            }
            if real_node.borrow().right.is_some() {
                self._print_custom_format(&real_node.borrow().right, &format!("{}      ", prefix), "R");
            }
        }
    }

    pub fn print_in_order(&self) {
        self._print_in_order(&self.root);
        println!();  // for a newline after the traversal
    }

    fn _print_in_order(&self, node: &RedBlackTree<T>) {
        if let Some(current) = node {
            self._print_in_order(&current.borrow().left);
            print!("{:?} ", current.borrow().key);  // print the current node's key followed by a space
            self._print_in_order(&current.borrow().right);
        }
    }

    pub fn is_empty(&self) -> bool {
        self.root.is_none()
    }
}

fn main() {
    let mut rb_tree = RedBlackTreeRoot::new();

    println!("Is tree empty? {}", rb_tree.is_empty());

    rb_tree.insert(10);
    rb_tree.insert(5);
    rb_tree.insert(20);
    rb_tree.insert(1);
    rb_tree.insert(6);
    rb_tree.insert(15);
    rb_tree.insert(25);

    // Checking if the tree is empty after some insertions
    println!("Is tree empty? {}", rb_tree.is_empty());

    rb_tree.delete(10);
    rb_tree.delete(5);
    rb_tree.delete(20);
    rb_tree.delete(1);
    rb_tree.delete(6);
    rb_tree.delete(15);
    rb_tree.delete(25);

    println!("Is tree empty? {}", rb_tree.is_empty());
}







