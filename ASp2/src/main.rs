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
    // 
    fn insert_rec(&self, node: AVLTreePtr<T>, new_node: AVLTreePtr<T>) -> Rc<RefCell<AVLNode<T>>> {
        if let Some(current_node) = node {
            let new_value = new_node.as_ref().unwrap().borrow().value.clone();
            
            {
                let mut current_node_ref = current_node.borrow_mut();
                if new_value < current_node_ref.value {
                    let left_child = current_node_ref.left.take();
                    current_node_ref.left = Some(self.insert_rec(left_child, new_node.clone()));
                } else if new_value > current_node_ref.value {
                    let right_child = current_node_ref.right.take();
                    current_node_ref.right = Some(self.insert_rec(right_child, new_node.clone()));
                } else {
                    return current_node.clone();
                }
            }
            current_node.borrow_mut().update_height();
            self.balance(current_node)
        } else {
            new_node.unwrap()
        }
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
        let taken_root = self.root.take();
        self.root = self.delete_rec(taken_root, value);
    }
    fn delete_rec(&self, node: AVLTreePtr<T>, value: T) -> AVLTreePtr<T> {
        if let Some(current_node) = node {
            let mut to_balance = None;
            {
                let mut node_borrow = current_node.borrow_mut();
    
                if value < node_borrow.value {
                    node_borrow.left = self.delete_rec(node_borrow.left.take(), value);
                } else if value > node_borrow.value {
                    node_borrow.right = self.delete_rec(node_borrow.right.take(), value);
                } else {
                    if node_borrow.left.is_some() && node_borrow.right.is_some() {
                        let temp = self.min_value_node(node_borrow.right.as_ref().unwrap().clone());
                        node_borrow.value = temp.borrow().value.clone();
                        node_borrow.right = self.delete_rec(node_borrow.right.take(), temp.borrow().value.clone());
                    } else if node_borrow.left.is_some() {
                        return node_borrow.left.take();
                    } else {
                        return node_borrow.right.take();
                    }
                }
        
                node_borrow.update_height();
                to_balance = Some(current_node.clone());
            }
            to_balance.map(|n| self.balance(n))
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
}

fn main() {
    let mut avl_tree = AVLTree::<i32>::new();
    avl_tree.insert(10);
    avl_tree.insert(5);
    avl_tree.insert(20);
    avl_tree.insert(1);
    avl_tree.insert(6);
    avl_tree.insert(15);
    avl_tree.insert(25);

    if avl_tree.is_empty() {
        println!("The AVL tree is empty.");
    } else {
        println!("The AVL tree is not empty.");
    }

    println!("In-order Traversal: {:?}", avl_tree.inorder_traversal());
    println!("Leaves Count: {}", avl_tree.count_leaves());
    println!("Height before deletion: {}", avl_tree.height());

    avl_tree.delete(10);
    avl_tree.delete(5);
    avl_tree.delete(20);
    println!("Leaves Count after deletion: {}", avl_tree.count_leaves());
    println!("Height after deletion: {}", avl_tree.height());
    println!("In-order Traversal after deletion: {:?}", avl_tree.inorder_traversal());
}