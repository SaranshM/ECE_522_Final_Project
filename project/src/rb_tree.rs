//! Red Black Tree
//!
//! An implementation of red black tree

use std::cell::RefCell;
use std::rc::Rc;
use std::fmt::{Debug, Display};

pub use crate::tree::{TreeTrait, TreeNodeTrait, Direction, SimpleTreeTrait, rotate, search_node};

/// Color of the nodes in red black tree
#[derive(Clone, Debug, PartialEq, Copy)]
pub enum NodeColor {
    Red,
    Black, 
}

#[derive(Clone, Debug, PartialEq)]
struct TreeNode<T: Ord+Copy+Debug+Display> {
    pub color: NodeColor,
    pub value: T,
    pub parent: TreeRoot<T>,
    left: TreeRoot<T>, 
    right: TreeRoot<T>
}
type TreeRoot<T> = Option<Rc<RefCell<TreeNode<T>>>>;

/// Struct of the red black tree
pub struct RedBlackTree<T: Ord+Copy+Debug+Display>{
    root: TreeRoot<T>
}


impl<T: Ord+Copy+Debug+Display> TreeTrait<T, TreeNode<T>> for RedBlackTree<T>{
    fn root(&self)->TreeRoot<T>{
        self.root.clone()
    }

    /// Check whether the red black tree is valid
    ///
    /// # Example
    ///
    /// ```
    /// use BinaryTress::rbtree::RedBlackTree;
    /// let mut rbtree: RedBlackTree<u32> = RedBlackTree::new();
    /// rbtree.insert(8);
    /// println!("{}", rbtree.check_valid());
    /// ```
    fn check_valid(&self)->bool{
        if self.root.is_none(){
            return true;
        }

        let root_nd = self.root.clone().unwrap();
        if root_nd.borrow().color != NodeColor::Black{
            println!("Root node should be black");
            return false;
        }
        let vec = self.in_order_traverse();
        let order = vec.iter().zip(vec.iter().skip(1))
            .all(|(current, next)| current<next);
        if !order{
            println!("Order error");
            return false;
        }
        if !root_nd.borrow().check_red_children(){
            println!("Red node doesn't have two black children");
            return false;
        }
        if !self.check_color(){
            println!("Black nodes in the paths don't agree");
            return false;
        }
        return true;
    }

    /// Helper for count_leaves()
    fn DEFAULT_LEAF_NUM(&self)->u32{
        2 as u32
    }
    /// Helper for height()
    fn DEFAULT_HEIGHT_NUM(&self)->u32{
        1 as u32
    }

}

impl<T: Ord+Copy+Debug+Display> SimpleTreeTrait<T> for RedBlackTree<T>{
    fn insert(&mut self, value: T)->bool{
        RedBlackTree::<T>::insert(self, value)
    }
    fn delete(&mut self, value: T)->Option<T>{
        RedBlackTree::<T>::delete(self, value)
    }
    fn count_leaves(&self)->u32{
        RedBlackTree::<T>::count_leaves(self)
    }
    fn is_empty(&self)->bool{
        RedBlackTree::<T>::is_empty(self)
    }
    fn print(&self, verbose: bool){
        RedBlackTree::<T>::print(self, verbose)
    }
    fn height(&self)->u32{
        RedBlackTree::<T>::height(self)
    }
    fn in_order_traverse(&self)->Vec<T>{
        RedBlackTree::<T>::in_order_traverse(self)
    }
}

impl<T: Ord+Copy+Debug+Display> RedBlackTree <T>{

    fn check_color(&self)->bool{
        if self.root.is_none(){
            return true;
        }
        let height_option = self.root.clone().unwrap().borrow().check_color();
        return height_option.is_some();
    }

    /// Create a new RedBlackTree
    ///
    /// # Example
    ///
    /// ```
    /// use BinaryTress::rbtree::RedBlackTree;
    /// let mut rbtree: RedBlackTree<u32> = RedBlackTree::new();
    /// ```
    pub fn new()->Self{
        RedBlackTree{root: None}
    }

    /// Delete a node in the RedBlackTree
    ///
    /// # Example
    ///
    /// ```
    /// use BinaryTress::rbtree::RedBlackTree;
    /// let mut rbtree: RedBlackTree<u32> = RedBlackTree::new();
    /// let deleted = rbtree.delete(8);
    /// println!("{:?}", deleted.is_none());
    /// ```
    pub fn delete(&mut self, value: T)->Option<T>{
        let node = search_node(self.root.clone(), value);
        if node.is_none(){
            return None;
        }
        let new_root = delete_node(node.clone().unwrap(), value);
        if new_root.is_some(){
            self.root = new_root.unwrap().clone();
        }
        return Some(value);
    }

    /// Insert a node to the AVLTree
    ///
    /// # Panic
    /// Illegal cases for rotation
    ///
    /// # Example
    ///
    /// ```
    /// use BinaryTress::rbtree::RedBlackTree;
    /// let mut rbtree: RedBlackTree<u32> = RedBlackTree::new();
    /// let inserted = rbtree.insert(8);
    /// println!("{}", rbtree.search(8));
    /// ```
    pub fn insert(&mut self, value:T)->bool{
        let root=self.root.clone();
        let mut res=false;
        self.root=match root {
            Some(root) => {
                let (new_root,res)=insert_node(root,value);
                new_root
            }
            None => {
                let mut new_node=TreeNode::new(value);
                new_node.color=NodeColor::Black;
                let new_root=Rc::new(RefCell::new(new_node));
                res=true;
                Some(new_root)
            },
        };
        res 
    }

    // repeating
    /// Check if the RedBlackTree is empty
    ///
    /// # Example
    ///
    /// ```
    /// use BinaryTress::rbtree::RedBlackTree;
    /// let mut rbtree: RedBlackTree<u32> = RedBlackTree::new();
    /// rbtree.insert(8);
    /// println!("{}", rbtree.is_empty());
    /// ```
    fn is_empty(&self)->bool{
        TreeTrait::<T, TreeNode<T>>::is_empty(self)
    }

    /// Count number of leaves in the RedBlackTree
    ///
    /// # Example
    ///
    /// ```
    /// use BinaryTress::rbtree::RedBlackTree;
    /// let mut rbtree: RedBlackTree<u32> = RedBlackTree::new();
    /// rbtree.insert(8);
    /// println!("{}", rbtree.count_leaves());
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
    /// use BinaryTress::rbtree::RedBlackTree;
    /// let mut rbtree: RedBlackTree<u32> = RedBlackTree::new();
    /// rbtree.insert(8);
    /// println!("{}", rbtree.print(true));
    /// ```
    fn print(&self, verbose: bool){
        TreeTrait::<T, TreeNode<T>>::print(self, verbose)
    }

    /// Get height of the RedBlackTree
    ///
    /// # Example
    ///
    /// ```
    /// use BinaryTress::rbtree::RedBlackTree;
    /// let mut rbtree: RedBlackTree<u32> = RedBlackTree::new();
    /// rbtree.insert(8);
    /// println!("{}", rbtree.height());
    /// ```
    fn height(&self)->u32{
        TreeTrait::<T, TreeNode<T>>::height(self)
    }
    /// In-order traverse of the tree
    ///
    /// The output will be a sorted vector
    ///
    /// # Example
    ///
    /// ```
    /// use BinaryTress::rbtree::RedBlackTree;
    /// let mut rbtree: RedBlackTree<u32> = RedBlackTree::new();
    /// rbtree.insert(8);
    /// rbtree.insert(10);
    /// println!("{}", rbtree.in_order_traverse());
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
        let cl = match self.color{
            NodeColor::Red=>"",
            NodeColor::Black=>"b"
        }.to_string();
        return val+&cl;
    }

    fn fmt_info(&self)->String{
        format!(
            "(Color: {:?}, Value: {:?}, Is Leaf: {:?})",
            self.color,
            self.value,
            self.is_leaf(),
        )
    }

}

impl<T: Ord+Copy+Debug+Display> TreeNode <T>{
    fn new(value: T) -> Self {
        TreeNode {
            color: NodeColor::Red,
            value: value,
            parent: None,
            left: None,
            right: None,
        }
    }

    fn new_with_parent(value: T, parent: Rc<RefCell<TreeNode<T>>>) -> Self {
        TreeNode {
            color: NodeColor::Red,
            value: value,
            parent: Some(parent),
            left: None,
            right: None,
        }
    }

    fn check_color(&self)->Option<usize>{
        let left_height = if let Some(ln)=self.left.clone(){
            ln.borrow().check_color()
        }
        else{
            Some(0)
        };
        let right_height = if let Some(rn)=self.right.clone(){
            rn.borrow().check_color()
        }
        else{
            Some(0)
        };
        match (left_height, right_height){
            (Some(lh), Some(rh))=>{
                if lh != rh{
                    return None;
                }
                match &self.color{
                    NodeColor::Red => Some(lh),
                    NodeColor::Black => Some(lh+1),
                } 
            },
            _=>None
        }
    }

    fn check_red_children(&self)->bool{
        let current = match &self.color{
            NodeColor::Red=>
            match (self.left.is_some(), self.right.is_some()){
                (false, false)=>true,
                (true, true)=>{
                    let lc = self.left.clone().unwrap().borrow().color;
                    let rc = self.right.clone().unwrap().borrow().color;
                    lc == NodeColor::Black && rc == NodeColor::Black
                },
                _=>false
            },
            NodeColor::Black=>true
        };
        if !current{
            return false;
        }
        if self.left.is_some(){
            if !self.left.clone().unwrap().borrow().check_red_children(){
                return false;
            }
        }
        if self.right.is_some(){
            if !self.right.clone().unwrap().borrow().check_red_children(){
                return false;
            }
        }
        return true;
    }

    pub fn is_red(node:TreeRoot<T>)->bool{
        if node.is_none(){
            //println!("uncle is none");
            return false;
        }else{
            let unwraped_node=node.clone().unwrap();
            if unwraped_node.borrow().color==NodeColor::Red{
                return true;
            }else{
                return false;
            }
        }
    }
    fn set_red(node:Rc<RefCell<TreeNode<T>>>) -> Rc<RefCell<TreeNode<T>>> {
        node.borrow_mut().color = NodeColor::Red;
        return node;
    }

    fn set_black(node:Rc<RefCell<TreeNode<T>>>) -> Rc<RefCell<TreeNode<T>>> {
        node.borrow_mut().color = NodeColor::Black;
        return node;
    }

    pub fn get_root(node:Rc<RefCell<TreeNode<T>>>)-> TreeRoot<T>{
        let parent=node.borrow().parent.clone();
        match parent {
            Some(p) => {
                Self::get_root(p)
            },
            None => Some(node),                                                                  
        }
    }


}
fn insert_node<T: Ord+Copy+Debug+Display>(node:Rc<RefCell<TreeNode<T>>>, value: T) -> (TreeRoot<T>,bool){
    //println!("insert {:?}",value);
    if node.borrow().value ==value{
        return (Some(node),false);
    }else if node.borrow().value >value{
        let left=node.borrow().left.clone();
        match left {
            Some(left_node) => {
                insert_node(left_node,value);
            }
            None => {
                node.borrow_mut().left= Some(Rc::new(RefCell::new(TreeNode::new_with_parent(value, node.clone()))));
                let left=node.borrow().left.clone().unwrap();
                insert_recolor(left);
            },
        }
    }else {
        let right=node.borrow().right.clone();
        match right {
            Some(right_node) => {
                insert_node(right_node,value);
            }
            None => {
                node.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new_with_parent(value, node.clone()))));
                let right=node.borrow().right.clone().unwrap();
                insert_recolor(right);
            },
        }
    }
    return (TreeNode::get_root(node),true);
}
fn insert_recolor<T: Ord+Copy+Debug+Display>(node:Rc<RefCell<TreeNode<T>>>){

    let parent=node.borrow().parent.clone();
    match parent {
        Some(parent) =>{
            //1.if parent is black, no need to change
            if(parent.borrow().color==NodeColor::Black){
                ();
            }
            //2.if parent is red
            else{
                let grand_parent=parent.borrow().parent.clone();
                match grand_parent {
                    Some(grand_parent) => {
                        if(grand_parent.borrow().color==NodeColor::Red){
                            panic!("insert into tree, red parent node have red grand parent.");
                        }
                        let parent_dir=parent.borrow().get_direction_to_parent();
                        let node_dir=node.borrow().get_direction_to_parent();
                        //2.1 RR
                        if parent_dir==Direction::Right&&node_dir==Direction::Right{
                            //println!("RR");
                            let uncle=grand_parent.borrow().left.clone();
                            //2.1.1 uncle=none||black               
                            if !TreeNode::is_red(uncle.clone()){                             
                                //grand parent node perform left rotation                              
                                rotate(&Some(grand_parent.clone()),&Some(parent.clone()));
                                //recolor parent to black and left sibling to red                                                               
                                let parent=node.borrow().parent.clone().unwrap();
                                TreeNode::set_black(parent.clone());
                                let left_sibling=parent.borrow().left.clone().unwrap();                                
                                TreeNode::set_red(left_sibling.clone());
                            }
                            //2.1.2 uncle=red
                            else{
                                //set parent and uncel to black
                                TreeNode::set_black(parent.clone());
                                let unwraped_uncle=uncle.clone().unwrap();
                                TreeNode::set_black(unwraped_uncle.clone());
                                //set grand to red and recolor
                                TreeNode::set_red(grand_parent.clone());
                                insert_recolor(grand_parent.clone());
                            }
                        }
                        //2.2 LL
                        else if node_dir==Direction::Left&&parent_dir==Direction::Left{
                            //println!("LL");
                            let uncle=grand_parent.borrow().right.clone();
                            //2.2.1 uncle=none||black
                            if !TreeNode::is_red(uncle.clone()){
                                //grand parent node perform right rotation
                                rotate(&Some(grand_parent.clone()),&Some(parent.clone()));
                                //recolor parent to black and right sibling to red                                
                                let parent=node.borrow().parent.clone().unwrap();
                                TreeNode::set_black(parent.clone());
                                let right_sibling=parent.borrow().right.clone().unwrap();                                
                                TreeNode::set_red(right_sibling.clone());
                            }
                            //2.2.2 uncle=red
                            else{
                                //set parent and uncel to black
                                TreeNode::set_black(parent.clone());
                                let unwraped_uncle=uncle.clone().unwrap();
                                TreeNode::set_black(unwraped_uncle.clone());
                                //set grand to red and recolor
                                TreeNode::set_red(grand_parent.clone());
                                insert_recolor(grand_parent.clone());
                            }
                        }
                        //2.3 LR
                        else if parent_dir==Direction::Left&&node_dir==Direction::Right{
                            //println!("LR");
                            let uncle=grand_parent.borrow().right.clone();
                            //2.3.1 uncle=none||black
                            if !TreeNode::is_red(uncle.clone()){
                                //left rotate parent to change LR condition into LL
                                rotate(&Some(parent.clone()),&Some(node.clone()));
                                //now node is the parent and we take the original parent, which is the left child now as a new inserted node
                                let left_child=node.borrow().left.clone().unwrap();
                                insert_recolor(left_child.clone());
                            }
                            //2.3.2 uncle=red
                            else{
                                //set parent and uncel to black
                                TreeNode::set_black(parent.clone());
                                let unwraped_uncle=uncle.clone().unwrap();
                                TreeNode::set_black(unwraped_uncle.clone());
                                //set grand to red and recolor
                                TreeNode::set_red(grand_parent.clone());
                                insert_recolor(grand_parent.clone());
                            }
                        }
                        //2.4 RL
                        else if parent_dir==Direction::Right&&node_dir==Direction::Left{
                            //println!("RL");
                            let uncle=grand_parent.borrow().left.clone();
                            //2.4.1 uncle=none||black
                            if !TreeNode::is_red(uncle.clone()){
                                //right rotate parent to change LR condition into LL
                                rotate(&Some(parent.clone()),&Some(node.clone()));
                                //now node is the parent and we take the original parent, which is the right child now as a new inserted node
                                let right_child=node.borrow().right.clone().unwrap();
                                insert_recolor(right_child.clone());
                            }
                            //2.4.2 uncle=red
                            else{
                                //set parent and uncel to black
                                TreeNode::set_black(parent.clone());
                                let unwraped_uncle=uncle.clone().unwrap();
                                TreeNode::set_black(unwraped_uncle.clone());
                                //set grand to red and recolor
                                TreeNode::set_red(grand_parent.clone());
                                insert_recolor(grand_parent.clone());
                            }
                        } 
                    }
                    None => {
                        //2.5 parent is root, set parent to black
                        TreeNode::set_black(parent);
                    }
                }
            }
        },
        //3. node is root
        None => {
            TreeNode::set_black(node);
        },
    }
}

fn delete_node<T: Ord+Copy+Debug+Display>(
    root: TreeRoot<T>, value: T)->Option<TreeRoot<T>>{
    if root.is_none(){
        return None;
    }

    let node = root.clone().unwrap();

    // Case0.1: Two children
    // => like BSTree
    if node.borrow().left.is_some() && node.borrow().right.is_some(){
        let right_min = node.borrow().right.clone().unwrap().borrow().get_min();
        let rchild = node.borrow().right.clone();
        let r = delete_node(rchild, right_min);
        node.borrow_mut().value = right_min;
        return r;
    }
    // else: no child; one child
    

    // Case0.2: No child
    // red=>just delete it
    if node.borrow().left.is_none() && node.borrow().right.is_none() && node.borrow().color == NodeColor::Red{
        let ret = node.borrow_mut().delete_node();
        return ret;
    }
    //else: one child&&black; one child

    // Case1: current node is red
    // Red case ends
    // Because red nodes must have 0 or 2 black children
    // else: current is black && one child; current black && no child

    // Case2: current black && one child
    // Case2.1: current is black && unique child is red
    // => Replace it with its red child
    let (child, _direction) = node.borrow().get_child_delete_helper();
    if child.is_some(){
        if child.clone().unwrap().borrow().color == NodeColor::Red{
            child.clone().unwrap().borrow_mut().color = NodeColor::Black;
            let ret = node.borrow_mut().delete_node();
            return ret;
        }
        else{
            // current black && unique child black=>invalid case;
            panic!("Error! If current node is black, its unique child cannot be black!");
        }
    }
    // else: 
    // current black && no child

    // Case3: 
    let ret0 = delete_rebalance_helper(root.clone());
    let ret = node.borrow_mut().delete_node();
    match &ret{
        None=>ret0,
        _=> ret
    }
}

fn delete_rebalance_helper<T: Ord+Copy+Debug+Display>(root: TreeRoot<T>)->Option<TreeRoot<T>> {
    if root.is_none(){
        return None;
    }

    let node = root.clone().unwrap();

    let mut new_root_ret: Option<TreeRoot<T>> = None;
    // Case3: current black && no child
    // Case3.1: child is new root
    // => node is root => finished 
    if node.borrow().parent.is_none(){
        return None; 
    }
    let direction = node.borrow().get_direction_to_parent();
    // First replace current node with its child N
    // Case3.2: sibling is red
    // =>sibling to black; parent to red; rotate 
    let mut sibling = match direction{
        Direction::Left=>node.borrow().parent.clone().unwrap().borrow_mut().right.clone().unwrap(),
        Direction::Right=>node.borrow().parent.clone().unwrap().borrow_mut().left.clone().unwrap(),
    };
    let sib_direction = direction.opposite();

    if sibling.borrow().color==NodeColor::Red{
        sibling.borrow_mut().color = NodeColor::Black;
        node.borrow().parent.clone().unwrap().borrow_mut().color = NodeColor::Red;
        rotate(&node.borrow().parent, &Some(sibling.clone()));
        if sibling.borrow().parent.is_none(){
            new_root_ret = Some(Some(sibling.clone()));
        }

        // sibling changed due to rotation
        sibling = match direction{
            Direction::Left=>node.borrow().parent.clone().unwrap().borrow().right.clone().unwrap(),
            Direction::Right=>node.borrow().parent.clone().unwrap().borrow().left.clone().unwrap(),
        };
    }

    // continue in Case3.4, 3.5, 3.6
    // else: sibling is black
    assert!(sibling.borrow().color == NodeColor::Black);
    // Case3.3&3.4: sibling black and black children or no child;
    let sib_left = sibling.borrow().left.clone();
    let sib_right = sibling.borrow().right.clone();
    if sib_left.is_some()&&sib_right.is_some()&&sib_left.clone().unwrap().borrow().color == NodeColor::Black && sib_right.clone().unwrap().borrow().color == NodeColor::Black ||
    sib_left.is_none()&&sib_right.is_none(){
        sibling.borrow_mut().color = NodeColor::Red;
        let par_color = node.borrow().parent.clone().unwrap().borrow().color;
        match par_color{
            // Case 3.3
            NodeColor::Black=>{
                let parent = node.borrow().parent.clone();
                let r = delete_rebalance_helper(parent);
                return r;
            },
            // Case 3.4
            NodeColor::Red=>{
                node.borrow().parent.clone().unwrap().borrow_mut().color = NodeColor::Black;
                return new_root_ret;
            }
        }
    }
    //else: sibling black && either child is red 

    if sibling.borrow().left.is_some() && sibling.borrow().right.is_some() &&
        sibling.borrow().left.clone().unwrap().borrow().color == NodeColor::Red && 
        sibling.borrow().right.clone().unwrap().borrow().color == NodeColor::Red {
        //panic!("not implemented");
        println!("unsafe case");
    }

    // Case3.5: sibling close child is red
    // => rotate, change color
    let sib_close_child = match sib_direction{
        Direction::Left=>sibling.borrow().right.clone(),
        Direction::Right=>sibling.borrow().left.clone(),
    };

    if sib_close_child.is_some() && sib_close_child.clone().unwrap().borrow().color == NodeColor::Red{
        rotate(&Some(sibling.clone()), &sib_close_child);
        if sib_close_child.clone().unwrap().borrow().parent.is_none(){
            new_root_ret = Some(sib_close_child.clone());
        }
        let sp_cl = sibling.borrow().parent.clone().unwrap().borrow().color;
        sibling.borrow_mut().color = sp_cl; 
        sibling.borrow().parent.clone().unwrap().borrow_mut().color = NodeColor::Black;
        // sibling changed due to rotation
        sibling = match direction{
            Direction::Left=>node.borrow().parent.clone().unwrap().borrow().right.clone().unwrap(),
            Direction::Right=>node.borrow().parent.clone().unwrap().borrow().left.clone().unwrap(),
        };
    }


    // Case3.6: sibling distant child is red 
    // =>  rotate, change color
    let sib_dist_child = match sib_direction{
        Direction::Left=>sibling.borrow().left.clone(),
        Direction::Right=>sibling.borrow().right.clone(),
    };
    if sib_dist_child.is_some() && sib_dist_child.clone().unwrap().borrow().color == NodeColor::Red{
        sib_dist_child.clone().unwrap().borrow_mut().color = NodeColor::Black;
        sibling.borrow_mut().color = node.borrow().parent.clone().unwrap().borrow().color;
        node.borrow().parent.clone().unwrap().borrow_mut().color = NodeColor::Black;
        rotate(&node.borrow().parent, &Some(sibling.clone()));
        if sibling.borrow().parent.is_none(){
            new_root_ret = Some(Some(sibling.clone()));
        }
    }
   return  new_root_ret;
}


// #[cfg(test)]
// mod test{
//     use crate::tree;

//     use super::*;
//     fn new_children(nd: &TreeRoot<i32>, lv:i32, rv:i32,lc: &str, rc:&str)-> (TreeRoot<i32>, TreeRoot<i32>){
//         let f = |s|if s == "r" {NodeColor::Red} else {NodeColor::Black};
//         let left: TreeNode<i32> = TreeNode{color: f(lc),
//         value:lv, parent: Some(Rc::clone(&nd.clone().unwrap())), left: None, right:None};
//         let left = Some(Rc::new(RefCell::new(left)));
//         let right: TreeNode<i32> = TreeNode{color: f(rc),
//         value:rv, parent: Some(Rc::clone(&nd.clone().unwrap())), left: None, right:None};
//         let right = Some(Rc::new(RefCell::new(right)));
//         return (left, right);
//     }
//     fn check_valid_delete(tree: &RedBlackTree<i32>, expect: Option<i32>, result: Option<i32>, pre_delete_vec: &mut Vec<i32>){
//         let mut vec = tree.in_order_traverse();
//         assert!(tree.check_valid());
//         match expect{
//             Some(value)=>{
//                 assert!(result.is_some() && result.clone().unwrap() == value);
//                 assert!(vec.len()==pre_delete_vec.len()-1);
//                 pre_delete_vec.retain(|&x| x != value);
//             },
//             None=>{
//                 assert!(result.is_none());
//             }
//         }
//         assert!(vec.iter().zip(&mut pre_delete_vec.iter()).filter(|&(a, b)| a != b).count()==0);
//     }
//     fn check_valid_insert(tree: &RedBlackTree<i32>, value: i32, pre_insert_vec: &mut Vec<i32>){
//         let mut vec = tree.in_order_traverse();
//         assert!(tree.check_valid());
//         assert!(vec.len()==pre_insert_vec.len()+1);
//         vec.retain(|&x| x != value);
//         assert!(vec.iter().zip(&mut pre_insert_vec.iter()).filter(|&(a, b)| a != b).count()==0);
//     }

//     #[test]
//     fn test_insert() {
//         let mut tree = RedBlackTree::new();
//         tree.insert(12);
//         tree.insert(1);
//         tree.insert(9);
//         tree.insert(2);
//         tree.insert(0);
//         tree.insert(11);
//         tree.insert(7);
//         tree.insert(19);
//         tree.insert(4);
//         tree.insert(15);
//         tree.insert(18);
//         tree.insert(5);
//         tree.insert(14);
//         tree.insert(13);
//         tree.insert(10);
//         tree.insert(16);
//         tree.insert(6);
//         tree.insert(3);
//         tree.insert(8);
//         tree.insert(17);
//         assert!(tree.check_valid());
//     }
//     #[test]
//     fn test_insert_RR_uncle_red() {
//         let mut tree = RedBlackTree::new();
//         tree.insert(1);
//         tree.insert(-1);
//         tree.insert(3);

//         let mut vec = tree.in_order_traverse();
//         tree.insert(4);
//         check_valid_insert(&tree, 4, &mut vec); 
//     }
//     #[test]
//     fn test_insert_RR_uncle_black() {
//         let mut tree = RedBlackTree::new();
//         tree.insert(5);
//         tree.insert(4);
//         tree.insert(9);
//         tree.insert(7);
//         tree.insert(6);

//         let mut vec = tree.in_order_traverse();
//         tree.insert(10);
//         check_valid_insert(&tree, 10, &mut vec); 
//     }
//     #[test]
//     fn test_insert_LL_uncle_red() {
//         let mut tree = RedBlackTree::new();
//         tree.insert(7);
//         tree.insert(4);
//         tree.insert(8);

//         let mut vec = tree.in_order_traverse();
//         tree.insert(3);
//         check_valid_insert(&tree, 3, &mut vec); 
//     }
//     #[test]
//     fn test_insert_LL_uncle_black() {
//         let mut tree = RedBlackTree::new();
//         tree.insert(11);
//         tree.insert(12);
//         tree.insert(4);
//         tree.insert(2);
//         tree.insert(9);
//         tree.insert(1);
//         tree.insert(10);

//         let mut vec = tree.in_order_traverse();
//         tree.insert(0);
//         check_valid_insert(&tree, 0, &mut vec); 
//     }
//     #[test]
//     fn test_insert_LR_uncle_red() {
//         let mut tree = RedBlackTree::new();
//         tree.insert(1);
//         tree.insert(2);
//         tree.insert(5);
         
//         let mut vec = tree.in_order_traverse();
//         tree.insert(3);
//         check_valid_insert(&tree, 3, &mut vec); 
        
//     }

//     #[test]
//     fn test_insert_LR_uncle_blalck() {
//         let mut tree = RedBlackTree::new();
//         tree.insert(4);
//         tree.insert(2);
//         tree.insert(1);
//         tree.insert(10);
//         tree.insert(8);
//         tree.insert(11);
         
//         let mut vec = tree.in_order_traverse();
//         tree.insert(9);
//         check_valid_insert(&tree, 9, &mut vec);         
//     }

//     #[test]
//     fn test_insert_RL_uncle_red() {
//         let mut tree = RedBlackTree::new();
//         tree.insert(2);
//         tree.insert(1);
//         tree.insert(4);
//         tree.insert(3);
//         tree.insert(7);
//         tree.insert(5);
//         tree.insert(9);
        
//         let mut vec = tree.in_order_traverse();
//         tree.insert(8);
//         check_valid_insert(&tree, 8, &mut vec);   
//     }

//     #[test]
//     fn test_insert_RL_uncle_black() {
//         let mut tree = RedBlackTree::new();
//         tree.insert(2);
//         tree.insert(1);
//         tree.insert(4);
//         tree.insert(3);
//         tree.insert(8);
//         tree.insert(6);
//         tree.insert(5);
//         tree.insert(7);
//         tree.insert(10);
        
//         let mut vec = tree.in_order_traverse();
//         tree.insert(9);
//         check_valid_insert(&tree, 9, &mut vec);   
//     }

    


//     #[test]
//     fn test_delete1(){
//         // root
//         let nd:TreeNode<i32> = TreeNode{color: NodeColor::Black,
//             value: 8, parent: None, left: None, right: None};
//         let nd = Some(Rc::new(RefCell::new(nd)));
//         let mut tree: RedBlackTree<i32> = RedBlackTree{root: nd.clone()};
//         let d = tree.delete(8);
//         assert!(d.is_some() && d.clone().unwrap() == 8);
//         assert!(tree.is_empty());
//     }

//     #[test]
//     fn test_delete2(){
//         // black+two children
//         let nd:TreeNode<i32> = TreeNode{color: NodeColor::Black,
//         value: 8, parent: None, left: None, right: None};
//         let nd = Some(Rc::new(RefCell::new(nd)));
//         let (left, right) = new_children(&nd, 2, 12, "b", "b");
        
//         let mut tree: RedBlackTree<i32> = RedBlackTree{root: nd.clone()};
//         let _nd = tree.root.clone().unwrap();
//         _nd.borrow_mut().left = left.clone();
//         _nd.borrow_mut().right = right.clone();
//         let mut vec = tree.in_order_traverse();
//         let d = tree.delete(8); 
//         check_valid_delete(&tree, Some(8), d, &mut vec); 
//     }

//     #[test]
//     fn test_delete3(){
//         //  red leaf
//         let nd:TreeNode<i32> = TreeNode{color: NodeColor::Black,
//         value: 8, parent: None, left: None, right: None};
//         let nd = Some(Rc::new(RefCell::new(nd)));
//         let (left, right) = new_children(&nd, 2, 12, "r", "b");
        
//         let mut tree: RedBlackTree<i32> = RedBlackTree{root: nd.clone()};
//         let _nd = tree.root.clone().unwrap();
//         _nd.borrow_mut().left = left.clone();

//         let mut vec = tree.in_order_traverse();
//         let d = tree.delete(2); 
//         check_valid_delete(&tree, Some(2), d, &mut vec); 
//     }

//     #[test]
//     fn test_delete4(){
//         // black + red sibling
//         let nd:TreeNode<i32> = TreeNode{color: NodeColor::Black,
//         value: 8, parent: None, left: None, right: None};
//         let nd = Some(Rc::new(RefCell::new(nd)));
//         let (left, right) = new_children(&nd, 2, 12, "b", "b");
//         let (rl, rr) = new_children(&right, 10, 20, "r", "b");
//         let (rll, rlr) = new_children(&rl, 9, 11, "b", "b");
//         let (ll, lr) = new_children(&left, 1, 5, "b", "b");
        
//         let mut tree: RedBlackTree<i32> = RedBlackTree{root: nd.clone()};
//         let _nd = tree.root.clone().unwrap();
//         _nd.borrow_mut().left = left.clone();
//         _nd.borrow_mut().right = right.clone();
        
//         let _right = right.clone().unwrap();
//         _right.borrow_mut().left = rl.clone();
//         _right.borrow_mut().right = rr.clone();
//         let _left = left.clone().unwrap();
//         _left.borrow_mut().left = ll.clone();
//         _left.borrow_mut().right = lr.clone();
//         let _rl = rl.clone().unwrap();
//         _rl.borrow_mut().left = rll.clone();
//         _rl.borrow_mut().right = rlr.clone();

//         let mut vec = tree.in_order_traverse();
//         let d = tree.delete(20);
//         check_valid_delete(&tree, Some(20), d, &mut vec);
//     }

//     #[test]
//     fn test_delete5(){
//         // black + black sibling + no nephew + red parent 
//         let nd:TreeNode<i32> = TreeNode{color: NodeColor::Black,
//         value: 8, parent: None, left: None, right: None};
//         let nd = Some(Rc::new(RefCell::new(nd)));
//         let (left, right) = new_children(&nd, 2, 12, "b", "b");
//         let (rl, rr) = new_children(&right, 10, 20, "r", "b");
//         let (rll, rlr) = new_children(&rl, 9, 11, "b", "b");
//         let (ll, lr) = new_children(&left, 1, 5, "b", "b");
        
//         let mut tree: RedBlackTree<i32> = RedBlackTree{root: nd.clone()};
//         let _nd = tree.root.clone().unwrap();
//         _nd.borrow_mut().left = left.clone();
//         _nd.borrow_mut().right = right.clone();
        
//         let _right = right.clone().unwrap();
//         _right.borrow_mut().left = rl.clone();
//         _right.borrow_mut().right = rr.clone();
//         let _left = left.clone().unwrap();
//         _left.borrow_mut().left = ll.clone();
//         _left.borrow_mut().right = lr.clone();
//         let _rl = rl.clone().unwrap();
//         _rl.borrow_mut().left = rll.clone();
//         _rl.borrow_mut().right = rlr.clone();


//         let mut vec = tree.in_order_traverse();
//         let d = tree.delete(9);
//         check_valid_delete(&tree, Some(9), d, &mut vec);
//     }

//     #[test]
//     fn test_delete6(){
//         // black + black sibling + no nephew + black parent 
//         let nd:TreeNode<i32> = TreeNode{color: NodeColor::Black,
//         value: 8, parent: None, left: None, right: None};
//         let nd = Some(Rc::new(RefCell::new(nd)));
//         let (left, right) = new_children(&nd, 2, 12, "b", "b");
        
//         let mut tree: RedBlackTree<i32> = RedBlackTree{root: nd.clone()};
//         let _nd = tree.root.clone().unwrap();
//         _nd.borrow_mut().left = left.clone();
//         _nd.borrow_mut().right = right.clone();

//         let mut vec = tree.in_order_traverse();
//         let d = tree.delete(2);
//         check_valid_delete(&tree, Some(2), d, &mut vec);
//     }
    
//     #[test]
//     fn test_delete7(){
//         // black + black sibling + close red nephew
//         let nd:TreeNode<i32> = TreeNode{color: NodeColor::Black,
//         value: 8, parent: None, left: None, right: None};
//         let nd = Some(Rc::new(RefCell::new(nd)));
//         let (left, right) = new_children(&nd, 2, 12, "b", "b");
//         let (rl, rr) = new_children(&right, 10, 20, "r", "b");
        
//         let mut tree: RedBlackTree<i32> = RedBlackTree{root: nd.clone()};
//         let _nd = tree.root.clone().unwrap();
//         _nd.borrow_mut().left = left.clone();
//         _nd.borrow_mut().right = right.clone();
        
//         let _right = right.clone().unwrap();
//         _right.borrow_mut().left = rl.clone();

//         let mut vec = tree.in_order_traverse();
//         let d = tree.delete(2);
//         check_valid_delete(&tree, Some(2), d, &mut vec);
//     }

//     #[test]
//     fn test_delete8(){
//         // black + black sibling + distant red nephew
//         let nd:TreeNode<i32> = TreeNode{color: NodeColor::Black,
//         value: 8, parent: None, left: None, right: None};
//         let nd = Some(Rc::new(RefCell::new(nd)));
//         let (left, right) = new_children(&nd, 2, 12, "b", "b");
//         let (rl, rr) = new_children(&right, 10, 20, "b", "r");
        
//         let mut tree: RedBlackTree<i32> = RedBlackTree{root: nd.clone()};
//         let _nd = tree.root.clone().unwrap();
//         _nd.borrow_mut().left = left.clone();
//         _nd.borrow_mut().right = right.clone();
        
//         let _right = right.clone().unwrap();
//         _right.borrow_mut().right = rr.clone();

//         let mut vec = tree.in_order_traverse();
//         let d = tree.delete(2);
//         check_valid_delete(&tree, Some(2), d, &mut vec);
//     }

// }