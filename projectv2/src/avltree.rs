//! AVL Tree
//!
//! An implementation of AVL Tree

use std::cell::RefCell;
use std::rc::Rc;
use std::fmt::{Debug, Display};
use std::cmp::max;


pub use crate::tree::{TreeTrait, TreeNodeTrait, Direction, SimpleTreeTrait};
use crate::tree::{rotate, search_node, search_insert_point};

#[derive(Clone, Debug, PartialEq)]
struct TreeNode<T: Ord+Copy+Debug+Display>{
    pub value: T,
    pub parent: TreeRoot<T>,
    left: TreeRoot<T>, 
    right: TreeRoot<T>,
    height: u32
}

type TreeRoot<T> = Option<Rc<RefCell<TreeNode<T>>>>;

/// Struct of AVLTree
///
/// connected with private struct of AVL tree node
#[derive(Clone, Debug, PartialEq)]
pub struct AVLTree<T: Ord+Copy+Debug+Display> {
    root: TreeRoot<T>
}

impl<T: Ord+Copy+Debug+Display> TreeTrait<T, TreeNode<T>> for AVLTree<T>{
    fn root(&self)->TreeRoot<T>{
        self.root.clone()
    }

    /// Check whether the AVL tree is valid
    ///
    /// # Example
    ///
    /// ```
    /// use BinaryTress::avltree::AVLTree;
    /// let mut avltree: AVLTree<u32> = AVLTree::new();
    /// avltree.insert(8);
    /// println!("{}", avltree.check_valid());
    /// ```
    fn check_valid(&self)->bool{
        if self.root.is_none(){
            return true;
        }
        let vec = self.in_order_traverse();
        let order = vec.iter().zip(vec.iter().skip(1))
            .all(|(current, next)| current<next);
        if !order{
            println!("Order error");
            return false;
        }

        self.root.clone().unwrap().borrow().is_balanced()
    }
}

impl<T: Ord+Copy+Debug+Display> SimpleTreeTrait<T> for AVLTree<T>{
    fn insert(&mut self, value: T)->bool{
        AVLTree::<T>::insert(self, value)
    }
    fn delete(&mut self, value: T)->Option<T>{
        AVLTree::<T>::delete(self, value)
    }
    fn count_leaves(&self)->u32{
        AVLTree::<T>::count_leaves(self)
    }
    fn is_empty(&self)->bool{
        AVLTree::<T>::is_empty(self)
    }
    fn print(&self, verbose: bool){
        AVLTree::<T>::print(self, verbose)
    }
    fn height(&self)->u32{
        AVLTree::<T>::height(self)
    }
    fn in_order_traverse(&self)->Vec<T>{
        AVLTree::<T>::in_order_traverse(self)
    }
}

impl <T: Ord+Copy+Debug+Display> AVLTree<T>{
    /// Create a new AVLTree
    ///
    /// # Example
    ///
    /// ```
    /// use BinaryTress::avltree::AVLTree;
    /// let mut avltree: AVLTree<u32> = AVLTree::new();
    /// ```
    pub fn new()->Self{
        AVLTree{root: None}
    }


    /// Delete a node in the AVLTree
    ///
    /// # Example
    ///
    /// ```
    /// use BinaryTress::avltree::AVLTree;
    /// let mut avltree: AVLTree<u32> = AVLTree::new();
    /// let deleted = avltree.delete(8);
    /// println!("{:?}", deleted.is_none());
    /// ```
    pub fn delete(&mut self, value: T)->Option<T>{
        let node = search_node(self.root.clone(), value);
        if node.is_none(){
            return None;
        }

        let (deleted, new_root) = delete_node(node.unwrap(), value);
        if new_root.is_some(){
            self.root = new_root.unwrap().clone();
        }
        return deleted;
    }

    /// Insert a node to the AVLTree
    ///
    /// # Panic
    /// Illegal cases for rotation
    ///
    /// # Example
    ///
    /// ```
    /// use BinaryTress::avltree::AVLTree;
    /// let mut avltree: AVLTree<u32> = AVLTree::new();
    /// let inserted = avltree.insert(8);
    /// println!("{}", avltree.search(8));
    /// ```
    pub fn insert(&mut self, value:T)->bool{
        if self.root.is_none(){
            self.root = TreeNode::new_root(value);
            return true;
        }
        let parent = search_insert_point(self.root.clone(), value);
        if parent.is_none(){
            return false;
        }
        let parent_nd = parent.clone().unwrap();
        let nd = TreeNode::new_root(value);
        nd.clone().unwrap().borrow_mut().set_parent(parent.clone());
        if value > parent_nd.borrow().value{
            parent_nd.borrow_mut().set_right(nd.clone());
        }
        else{
            parent_nd.borrow_mut().set_left(nd.clone());
        }

        let new_root = rebalance_helper(parent.clone());
        if new_root.is_some(){
            let new_root = new_root.unwrap();
            let new_root_val = new_root.clone().unwrap().borrow().value;
            let root_val = self.root.clone().unwrap().borrow().value;
            if new_root_val != root_val{
                self.root = new_root.clone();
            }
        }

        return true;

    }

    /// Get height of the AVLTree
    ///
    /// # Example
    ///
    /// ```
    /// use BinaryTress::avltree::AVLTree;
    /// let mut avltree: AVLTree<u32> = AVLTree::new();
    /// avltree.insert(8);
    /// println!("{}", avltree.height());
    /// ```
    pub fn height(&self)->u32{
        match self.root.clone(){
            None=>0,
            Some(r)=>r.borrow().height()
        }

    }

    // repeating
    /// Check if the AVLTree is empty
    ///
    /// # Example
    ///
    /// ```
    /// use BinaryTress::avltree::AVLTree;
    /// let mut avltree: AVLTree<u32> = AVLTree::new();
    /// avltree.insert(8);
    /// println!("{}", avltree.is_empty());
    /// ```
    fn is_empty(&self)->bool{
        TreeTrait::<T, TreeNode<T>>::is_empty(self)
    }
    /// Count number of leaves in the AVLTree
    ///
    /// # Example
    ///
    /// ```
    /// use BinaryTress::avltree::AVLTree;
    /// let mut avltree: AVLTree<u32> = AVLTree::new();
    /// avltree.insert(8);
    /// println!("{}", avltree.count_leaves());
    /// ```
    fn count_leaves(&self)->u32{
        TreeTrait::<T, TreeNode<T>>::count_leaves(self)
    }
    /// Print the information of the tree
    ///
    /// Print the tree structure;
    ///
    /// Additional verbose information of the tree if verbose is true.
    ///
    /// # Example
    ///
    /// ```
    /// use BinaryTress::avltree::AVLTree;
    /// let mut avltree: AVLTree<u32> = AVLTree::new();
    /// avltree.insert(8);
    /// println!("{}", avltree.print(true));
    /// ```
    fn print(&self, verbose: bool){
        TreeTrait::<T, TreeNode<T>>::print(self, verbose)
    }
    /// In-order traverse of the tree
    ///
    /// The output will be a sorted vector
    ///
    /// # Example
    ///
    /// ```
    /// use BinaryTress::avltree::AVLTree;
    /// let mut avltree: AVLTree<u32> = AVLTree::new();
    /// avltree.insert(8);
    /// avltree.insert(10);
    /// println!("{}", avltree.in_order_traverse());
    /// ```
    fn in_order_traverse(&self)->Vec<T>{
        TreeTrait::<T, TreeNode<T>>::in_order_traverse(self)
    }
}


impl<T: Ord+Copy+Debug+Display> TreeNodeTrait<T> for TreeNode <T>{
    fn left(&self)->TreeRoot<T>{
        self.left.clone()
    }
    fn right(&self)->TreeRoot<T>{
        self.right.clone()
    }
    fn parent(&self)->TreeRoot<T>{
        self.parent.clone()
    }
    fn value(&self)->T{
        self.value
    }

    fn set_left(&mut self, v: TreeRoot<T>){
        self.left = v
    }
    fn set_right(&mut self, v: TreeRoot<T>){
        self.right = v
    }
    fn set_parent(&mut self, v: TreeRoot<T>){
        self.parent = v
    }
    fn set_value(&mut self, v: T){
        self.value = v;
    }

    fn structure_info(&self)->String{
        let val = self.value.to_string();
        return val;
    }

    fn fmt_info(&self)->String{
        format!(
            "(Value: {:?}, Height: {:?}, Is Leaf: {:?})",
            self.value, self.height, self.is_leaf()
        )
    }
}

impl <T: Ord+Copy+Debug+Display> TreeNode<T>{

    fn new_root(value: T)->TreeRoot<T>{
        let nd = TreeNode{
            value: value,
            left: None,
            right: None,
            parent: None,
            height: 1
        };
        Some(Rc::new(RefCell::new(nd)))
    }
    fn height(&self)->u32{
        self.height
    }

    fn is_balanced(&self)->bool{
        let (left_height, right_height): (u32, u32) = self.get_children_height();
        i32::abs(left_height as i32 - right_height as i32) < (2 as i32)
    }

    fn get_children_height(&self)->(u32, u32){
        let left_height = if let Some(ln)=self.left.clone(){
            ln.borrow().height
        }
        else{
            0
        };
        let right_height = if let Some(rn)=self.right.clone(){
            rn.borrow().height
        }
        else{
            0
        };
        return (left_height, right_height);
    }

    fn update_height(&mut self){
        let (left_height, right_height) = self.get_children_height();
        self.height = max(left_height, right_height) + 1;
    }

    fn check_balance_recurse(&self)->bool{
        if self.left.is_some(){
            if !self.left.clone().unwrap().borrow().is_balanced(){
                return false;
            }
        }
        if self.right.is_some(){
            if !self.right.clone().unwrap().borrow().is_balanced(){
                return false;
            }
        }
        if !self.is_balanced(){
            return false;
        }
        return true;
    }
}


fn delete_node<T: Ord+Copy+Debug+Display>(root: TreeRoot<T>, value: T)->(Option<T>, Option<TreeRoot<T>>){
    if root.is_none(){
        return (None, None);
    }

    let mut root = root.clone();
    if root.clone().unwrap().borrow().value != value{
        let _root = search_node(root.clone(), value);
        if _root.is_none(){
            return (None, None);
        }
        root = _root.unwrap();
    }
    let node = root.clone().unwrap();
    
    // Two children
    // => like BSTree
    if node.borrow().left.is_some() && node.borrow().right.is_some(){
        let right_min = node.borrow().right.clone().unwrap().borrow().get_min();
        let rchild = node.borrow().right.clone();
        let (_v, r) = delete_node(rchild, right_min);
        node.borrow_mut().value = right_min;
        return (Some(value), r);
    }
    let parent = node.borrow_mut().parent.clone();
    let (child, _direction) = node.borrow().get_child_delete_helper();
    let ret0 = node.borrow_mut().delete_node();
    let ret = match child.is_some(){
        true=>rebalance_helper(child.clone()),
        false=>rebalance_helper(parent.clone())
    };
    let r = match &ret{
        None=>ret0,
        _=> ret
    };
    return (Some(value), r);
}

fn rebalance_helper<T: Ord+Copy+Debug+Display>(root: TreeRoot<T>)->Option<TreeRoot<T>> {
    if root.is_none(){
        return None;
    }
 
    let node = root.clone().unwrap();
    node.borrow_mut().update_height();
    if node.borrow().is_balanced(){
        let par = node.borrow().parent.clone();
        return rebalance_helper(par);
    }


    let (left_height, right_height) = node.borrow().get_children_height();
    if left_height<right_height{
        if node.borrow().right.is_none(){
            panic!("Error!");
        }
        let (rlh, rrh) = node.borrow().right.clone().unwrap().borrow().get_children_height();
        if rrh >= rlh{
            left_left_rotate(&root.clone());
        }
        else{
            left_right_rotate(&root.clone());
        }
    }
    else{
        if node.borrow().left.is_none(){
            panic!("Error!");
        }
        let (llh, lrh) = node.borrow().left.clone().unwrap().borrow().get_children_height();
        if llh >= lrh{
            right_right_rotate(&root.clone());
        }
        else{
            right_left_rotate(&root.clone());
        }
    }
    let ret = Some(node.borrow().parent.clone());
    let p = node.borrow().parent.clone();
    let retp = rebalance_helper(p);
    if retp.is_none(){
        return ret;
    }
    else{
        return retp;
    }
}

fn left_rotate<T: Ord+Copy+Debug+Display>(root: &TreeRoot<T>){
    let right = root.clone().unwrap().borrow().right.clone();
    rotate(&root, &right);
}

fn right_rotate<T: Ord+Copy+Debug+Display>(root: &TreeRoot<T>){
    let left = root.clone().unwrap().borrow().left.clone();
    rotate(&root, &left);
}

fn left_left_rotate<T: Ord+Copy+Debug+Display>(root: &TreeRoot<T>){
    left_rotate(&root);
    root.clone().unwrap().borrow_mut().update_height();
    root.clone().unwrap().borrow().parent.clone().unwrap().borrow_mut().update_height();
}

fn right_right_rotate<T: Ord+Copy+Debug+Display>(root: &TreeRoot<T>){
    right_rotate(&root);
    root.clone().unwrap().borrow_mut().update_height();
    root.clone().unwrap().borrow().parent.clone().unwrap().borrow_mut().update_height();
}

fn left_right_rotate<T: Ord+Copy+Debug+Display>(root: &TreeRoot<T>){
    let right = root.clone().unwrap().borrow().right();
    right_right_rotate(&right);
    left_left_rotate(&root);
}

fn right_left_rotate<T: Ord+Copy+Debug+Display>(root: &TreeRoot<T>){
    let left = root.clone().unwrap().borrow().left.clone();
    left_left_rotate(&left);
    right_right_rotate(&root);
}


#[cfg(test)]
mod test{
    use super::*;
    fn check_valid_insert(tree: &AVLTree<i32>, value: i32, pre_delete_vec: &mut Vec<i32>){
        let mut vec = tree.in_order_traverse();
        assert!(tree.check_valid());
        assert!(vec.len()==pre_delete_vec.len()+1);
        vec.retain(|&x| x != value);
        assert!(vec.iter().zip(&mut pre_delete_vec.iter()).filter(|&(a, b)| a != b).count()==0);
    }
    fn check_valid_delete(tree: &AVLTree<i32>, expect: Option<i32>, result: Option<i32>, pre_delete_vec: &mut Vec<i32>){
        let mut vec = tree.in_order_traverse();
        assert!(tree.check_valid());
        match expect{
            Some(value)=>{
                assert!(result.is_some() && result.clone().unwrap() == value);
                assert!(vec.len()==pre_delete_vec.len()-1);
                pre_delete_vec.retain(|&x| x != value);
            },
            None=>{
                assert!(result.is_none());
            }
        }
        assert!(vec.iter().zip(&mut pre_delete_vec.iter()).filter(|&(a, b)| a != b).count()==0);
    }

    #[test]
    fn test_insert1(){
        // insert new
        let mut tree: AVLTree<i32> = AVLTree{root: None};
        assert!(tree.insert(5));
        assert!(tree.root.is_some() && tree.root.clone().unwrap().borrow().value==5);
    }

    #[test]
    fn test_insert2(){
        // insert one
        let mut tree: AVLTree<i32> = AVLTree{root: None};
        tree.insert(5);
        let mut vec:Vec::<i32> = vec![5];
        assert!(tree.insert(8));
        check_valid_insert(&tree, 8, &mut vec);
    }

    #[test]
    fn test_insert3(){
        // RR
        let mut tree: AVLTree<i32> = AVLTree{root: None};
        tree.insert(5);
        tree.insert(6);
        let mut vec:Vec::<i32> = vec![5, 6];
        assert!(tree.insert(8));
        check_valid_insert(&tree, 8, &mut vec);
    }

    #[test]
    fn test_insert4(){
        // LL
        let mut tree: AVLTree<i32> = AVLTree{root: None};
        tree.insert(5);
        tree.insert(4);
        let mut vec:Vec::<i32> = vec![4, 5];
        assert!(tree.insert(3));
        check_valid_insert(&tree, 3, &mut vec);
    }

    #[test]
    fn test_insert5(){
        // LR
        let mut tree: AVLTree<i32> = AVLTree{root: None};
        tree.insert(5);
        tree.insert(2);
        let mut vec:Vec::<i32> = vec![2, 5];
        assert!(tree.insert(3));
        check_valid_insert(&tree, 3, &mut vec);
    }

    #[test]
    fn test_insert6(){
        // RL
        let mut tree: AVLTree<i32> = AVLTree{root: None};
        tree.insert(5);
        tree.insert(9);
        let mut vec:Vec::<i32> = vec![5, 9];
        assert!(tree.insert(6));
        check_valid_insert(&tree, 6, &mut vec);
    }

    #[test]
    fn test_insert7(){
        let mut tree: AVLTree<i32> = AVLTree{root: None};
        tree.insert(5);
        tree.insert(2);
        let mut vec:Vec::<i32> = vec![2, 5];
        assert!(tree.insert(8));
        check_valid_insert(&tree, 8, &mut vec);
    }


    #[test]
    fn test_insert8(){
        let mut tree: AVLTree<i32> = AVLTree{root: None};
        tree.insert(5);
        tree.insert(2);
        tree.insert(8);
        assert!(!tree.insert(8));
    }


    #[test]
    fn test_delete1(){
        // root
        let mut tree: AVLTree<i32> = AVLTree{root: None};
        tree.insert(8);
        let d = tree.delete(8);
        assert!(d.is_some() && d.clone().unwrap() == 8);
        assert!(tree.is_empty());
    }

    #[test]
    fn test_delete2(){
        // LL
        let mut tree: AVLTree<i32> = AVLTree{root: None};
        tree.insert(5);
        tree.insert(8);
        tree.insert(2);
        tree.insert(1);
        let mut vec = tree.in_order_traverse();
        let d = tree.delete(1);
        check_valid_delete(&tree, Some(1), d, &mut vec);
    }

    #[test]
    fn test_delete3(){
        // RR
        let mut tree: AVLTree<i32> = AVLTree{root: None};
        tree.insert(5);
        tree.insert(8);
        tree.insert(2);
        tree.insert(10);
        let mut vec = tree.in_order_traverse();
        let d = tree.delete(10);
        check_valid_delete(&tree, Some(10), d, &mut vec);
    }

    #[test]
    fn test_delete4(){
        // LR
        let mut tree: AVLTree<i32> = AVLTree{root: None};
        tree.insert(5);
        tree.insert(8);
        tree.insert(2);
        tree.insert(1);
        tree.insert(3);
        let mut vec = tree.in_order_traverse();
        let d = tree.delete(1);
        check_valid_delete(&tree, Some(1), d, &mut vec);
    }

    #[test]
    fn test_delete5(){
        // RL
        let mut tree: AVLTree<i32> = AVLTree{root: None};
        tree.insert(5);
        tree.insert(8);
        tree.insert(2);
        tree.insert(6);
        tree.insert(10);
        let mut vec = tree.in_order_traverse();
        tree.print(true);
        let d = tree.delete(10);
        tree.print(true);
        check_valid_delete(&tree, Some(10), d, &mut vec);
    }

    #[test]
    fn test_delete6(){
        // invalid
        let mut tree: AVLTree<i32> = AVLTree{root: None};
        tree.insert(8);
        tree.insert(80);
        assert!(tree.delete(25).is_none());
    }

    #[test]
    fn test_delete7(){
        // two children
        let mut tree: AVLTree<i32> = AVLTree{root: None};
        tree.insert(5);
        tree.insert(8);
        tree.insert(2);
        tree.insert(6);
        tree.insert(10);
        let mut vec = tree.in_order_traverse();
        tree.print(true);
        let d = tree.delete(5);
        tree.print(true);
        check_valid_delete(&tree, Some(5), d, &mut vec);
    }


}
