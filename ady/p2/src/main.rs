use std::rc::Rc;
use std::cell::RefCell;

type AVLTreePtr<T> = Option<Rc<RefCell<AVLNode<T>>>>;

#[derive(Debug, Clone)]
struct AVLNode<T: Ord + Clone> {
    value: T,
    left: AVLTreePtr<T>,
    right: AVLTreePtr<T>,
    height: isize,
}

#[derive(Debug)]
struct AVLTree<T: Ord + Clone> {
    root: AVLTreePtr<T>,
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

impl<T: Ord + Clone> AVLTree<T> {
    pub fn new() -> Self {
        AVLTree { root: None }
    }

    pub fn is_empty(&self) -> bool {
        self.root.is_none()
    }

    pub fn insert(&mut self, value: T) {
        let new_node = AVLNode::new(value);
        if self.root.is_none() {
            self.root = Some(new_node);
        } else {
            let taken_root = self.root.take();
            self.root = Some(self.insert_rec(taken_root, Some(new_node)));
        }
    }

    fn insert_rec(&self, node: AVLTreePtr<T>, new_node: AVLTreePtr<T>) -> Rc<RefCell<AVLNode<T>>> {
        let current_node = node.unwrap();
        let new_value = new_node.as_ref().unwrap().borrow().value.clone();

        if new_value < current_node.borrow().value {
            let left_child = current_node.borrow_mut().left.take();
            current_node.borrow_mut().left = Some(self.insert_rec(left_child, new_node));
        } else if new_value > current_node.borrow().value {
            let right_child = current_node.borrow_mut().right.take();
            current_node.borrow_mut().right = Some(self.insert_rec(right_child, new_node));
        } else {
            return current_node;
        }

        current_node.borrow_mut().update_height();
        self.balance(current_node)
    }

    fn balance(&self, node: Rc<RefCell<AVLNode<T>>>) -> Rc<RefCell<AVLNode<T>>> {
        let balance_factor = node.borrow().balance_factor();

        // Left heavy
        if balance_factor > 1 {
            if node.borrow().left.as_ref().unwrap().borrow().balance_factor() < 0 {
                let left_child = node.borrow_mut().left.take().unwrap();
                node.borrow_mut().left = Some(self.rotate_left(left_child));
            }
            return self.rotate_right(node);
        }

        // Right heavy
        if balance_factor < -1 {
            if node.borrow().right.as_ref().unwrap().borrow().balance_factor() > 0 {
                let right_child = node.borrow_mut().right.take().unwrap();
                node.borrow_mut().right = Some(self.rotate_left(right_child));
            }
            return self.rotate_left(node);
        }

        node
    }

    fn rotate_left(&self, x: Rc<RefCell<AVLNode<T>>>) -> Rc<RefCell<AVLNode<T>>> {
        let y = x.borrow_mut().right.take().unwrap();
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
        if let Some(root) = &self.root {
            self.root = Some(self.delete_rec(root.clone(), value));
        }
    }

    fn delete_rec(&self, node: Rc<RefCell<AVLNode<T>>>, value: T) -> Rc<RefCell<AVLNode<T>>> {
        let current_node = node;

        if value < current_node.borrow().value {
            if let Some(left_child) = &current_node.borrow().left {
                current_node.borrow_mut().left = Some(self.delete_rec(left_child.clone(), value));
            }
        } else if value > current_node.borrow().value {
            if let Some(right_child) = &current_node.borrow().right {
                current_node.borrow_mut().right = Some(self.delete_rec(right_child.clone(), value));
            }
        } else {
            if current_node.borrow().left.is_none() {
                return current_node.borrow_mut().right.take().unwrap_or_else(|| current_node.clone());
            } else if current_node.borrow().right.is_none() {
                return current_node.borrow_mut().left.take().unwrap_or_else(|| current_node.clone());
            }

            let temp = self.min_value_node(current_node.borrow().right.clone().unwrap());
            let temp_val = temp.borrow().value.clone();
            current_node.borrow_mut().value = temp_val.clone();
            current_node.borrow_mut().right = Some(self.delete_rec(temp.clone(), temp_val));
        }

        current_node.borrow_mut().update_height();
        self.balance(current_node.clone())
    }

    fn min_value_node(&self, node: Rc<RefCell<AVLNode<T>>>) -> Rc<RefCell<AVLNode<T>>> {
        let mut current = node.clone();
        while let Some(left) = current.borrow().left.clone() {
            current = left;
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
}

fn main() {
    let mut avl_tree = AVLTree::<i32>::new();
    avl_tree.insert(10);
    avl_tree.insert(20);
    avl_tree.insert(30);
    avl_tree.insert(40);
    avl_tree.insert(50);
    avl_tree.insert(25);

    println!("Leaves Count: {}", avl_tree.count_leaves());

    avl_tree.delete(25);
    println!("Leaves Count after deletion: {}", avl_tree.count_leaves());
}
