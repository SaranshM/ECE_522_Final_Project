//! Traits for binary trees
//!
//! Define traits for tree structs and tree node structs

use std::cell::RefCell;
use std::rc::Rc;
use std::cmp::max;

use std::fmt::{Debug, Display};

/// Enum of direction for binary trees
#[derive(Clone, Debug, PartialEq)]
pub enum Direction{
    Left,
    Right
}


impl Direction{
    /// Get the opposite direction
    ///
    /// Can be used to get a node's sibling
    ///
    /// # Example
    /// let l = Direction::Left;
    /// let r = l.opposite();
    pub fn opposite(&self)->Direction{
        match self{
            Direction::Left=>Direction::Right,
            Direction::Right=>Direction::Left
        }
    }

    /// Check if current node is Direction::Left
    ///
    /// # Example
    /// let l = Direction::Left;
    /// if l.is_left(){
    ///     println!("Left");
    /// }
    fn is_left(&self)->bool{
        match self{
            Direction::Left=>true,
            Direction::Right=>false
        }
    }
}



/// A simple and general trait for trees
///
/// All the trees in the lib implements it. So can be used for dynamic dispatch.
pub trait SimpleTreeTrait<T: Ord+Copy+Debug+Display>{
    
    fn insert(&mut self, value: T)->bool;
    fn delete(&mut self, value: T)->Option<T>;
    fn count_leaves(&self)->u32;
    fn is_empty(&self)->bool;
    fn print(&self, verbose: bool);
    fn height(&self)->u32;
    fn in_order_traverse(&self)->Vec<T>;

}

/// Trait for the binary trees
///
/// Should implement SimpleTreeTrait
pub trait TreeTrait<T: Ord+Copy+Debug+Display, TreeNode: TreeNodeTrait<T>>: SimpleTreeTrait<T>{
    
    /// Get the root of the tree
    fn root(&self)->Option<Rc<RefCell<TreeNode>>>;

    /// Count number of leaves in the tree
    fn count_leaves(&self)->u32{
        if self.root().is_some(){
            return self.root().unwrap().borrow().count_leaves();
        }
        return self.DEFAULT_LEAF_NUM();
    }
    /// Check whether the tree is empty
    fn is_empty(&self)->bool{
        self.root().is_none()
    }
    /// Print the information of the tree
    ///
    /// Print the tree structure;
    ///
    /// Additional verbose information of the tree if verbose is true. 
    fn print(&self, verbose: bool){
        let root = self.root();
        if root.is_none() {
            println!("Empty tree!");
            return;
        }
        if verbose{
            root.clone().unwrap().borrow().print_tree_as_fmt("  ".to_string());
        }
        root.unwrap().borrow().print_structure();
    }

    /// Get height of the AVLTree
    fn height(&self)->u32{
        match self.root(){
            None => self.DEFAULT_HEIGHT_NUM(),
            Some(node) => node.borrow().get_height(),
        }
    }

    /// In-order traverse of the tree
    ///
    /// The output will be a sorted vector
    fn in_order_traverse(&self)->Vec<T>{
        let mut result = Vec::<T>::new();
        let root = self.root();
        if root.is_some(){
            root.unwrap().borrow().inorder(&mut result);
        }
        return result;
    }

    /// Search a node in the Tree
    ///
    /// ```
    /// use BinaryTress::avltree::AVLTree;
    /// let mut avltree: AVLTree<u32> = AVLTree::new();
    /// let is_contain = avltree.search(8);
    /// ```
    fn search(&self, value: T)->bool{
        search_node(self.root(), value).is_some()
    }

    /// Check whether the tree is valid
    fn check_valid(&self)->bool;

    // assocated constants
    /// An associated value for count_leaves
    ///
    /// 0 for default and 2 for NIL nodes
    fn DEFAULT_LEAF_NUM(&self)->u32{
        0 as u32
    }

    /// An associated value for height
    ///
    /// 0 for default and 1 for NIL nodes
    fn DEFAULT_HEIGHT_NUM(&self)->u32{
        0 as u32
    }
}

/// Trait for the tree nodes
pub trait TreeNodeTrait<T: Ord+Copy+Debug+Display>{
    /// Get reference to left child
    fn left(&self)->Option<Rc<RefCell<Self>>>;
    /// Get reference to right child
    fn right(&self)->Option<Rc<RefCell<Self>>>;
    /// Get reference to parent
    fn parent(&self)->Option<Rc<RefCell<Self>>>;
    /// Get the nodes' value
    fn value(&self)->T;

    /// Set left child
    fn set_left(&mut self, v: Option<Rc<RefCell<Self>>>);
    /// Set right child
    fn set_right(&mut self, v: Option<Rc<RefCell<Self>>>);
    /// Set parent
    fn set_parent(&mut self, v: Option<Rc<RefCell<Self>>>);
    /// Set value
    fn set_value(&mut self, v: T);

    /// Get the minimum of its right sub-tree
    ///
    /// Used for deletion
    fn get_min(&self)->T{
        match &self.left(){
            None=>self.value(),
            Some(nd)=>{
                nd.borrow().get_min()
            }
        }
    }

    /// Delete a node
    ///
    /// Link its parent and children;
    /// Clear its reference
    fn delete_node(&mut self)->Option<Option<Rc<RefCell<Self>>>>{
        // deal nodes with 1 or 0 child
        assert!(!(self.left().is_some()&&self.right().is_some()));

        let child = match self.right(){
            None=>self.left(),
            Some(_)=>self.right()
        };

        if child.is_some(){
            child.clone().unwrap().borrow_mut().set_parent(self.parent());
        }
        let ret = match self.parent(){
            Some(parent)=>{
                let direction = self.get_direction_to_parent();
                match direction{
                    Direction::Left=>parent.borrow_mut().set_left(self.left()),
                    Direction::Right=>parent.borrow_mut().set_right(self.right())
                };
                None
            },
            None=>Some(child)
        };
        self.set_parent(None);
        self.set_left(None);
        self.set_right(None);
        return ret;
    }

    /// A helper function for deletion
    ///
    /// Get current node's child
    ///
    /// Assumption: current node has only one or no child
    ///
    /// # Panic
    /// if current node has two children
    fn get_child_delete_helper(&self)->(Option<Rc<RefCell<Self>>>, Direction){
        assert!(!(self.left().is_some()&&self.right().is_some()));
        // one child or no child
        if self.left().is_some(){
            return (self.left(), Direction::Left);
        }
        if self.right().is_some(){
            return (self.right(), Direction::Right);
        }
        return (None, Direction::Left);
    }
    
    /// Get whether current node is the left child of its parent or right
    ///
    /// # Panic
    /// parent is None
    fn get_direction_to_parent(&self)->Direction{
        assert!(self.parent().is_some());
        let p = self.parent().unwrap();
        let parent = p.borrow();
        assert!(parent.value() != self.value());
        if parent.value() > self.value(){
            Direction::Left
        }
        else{
            Direction::Right
        }
    }

    /// Get the sibling of current node
    fn get_sibling(&self)->Option<Rc<RefCell<Self>>>{
        if self.parent().is_none(){
            return None;
        }
        let p = self.parent().unwrap();
        let direc = self.get_direction_to_parent();
        match direc{
            Direction::Left=>p.borrow().right().clone(),
            Direction::Right=>p.borrow().left().clone(),
        }
    }
    
    /// Check whether current node is a leaf node
    fn is_leaf(&self)->bool{
        self.left().is_none() && self.right().is_none()
    }
    
    /// Print the verbose information of the node
    ///
    /// Helper of print()
    fn print_tree_as_fmt(&self, ident: String){
        println!("{}",self.fmt_info());

        if self.left().is_some() {
            print!("{}left: ", ident);
            self.left().unwrap().borrow().print_tree_as_fmt(ident.clone()+"  ");
        }

        if self.right().is_some() {
            print!("{}right: ", ident);
            self.right().unwrap().borrow().print_tree_as_fmt(ident.clone()+"  ");
        }
    }

    /// Helper of in_order_traverse()
    fn inorder(&self, result: &mut Vec<T>) {
        if self.left().is_some() {
            self.left().unwrap().borrow().inorder(result);
        }
        result.push(self.value());
        if self.right().is_some() {
            self.right().unwrap().borrow().inorder(result);
        }
    }

    /// Helper of height()
    fn get_height(&self)->u32{
        let left_height = if let Some(ln)=self.left().clone(){
            ln.borrow().get_height()
        }
        else{
            0
        };
        let right_height = if let Some(rn)=self.right().clone(){
            rn.borrow().get_height()
        }
        else{
            0
        };

        return max(left_height, right_height)+1;
    }

    /// Helper to print the stucture of the tree as a tree
    fn print_structure(&self){
        let height = self.get_height() as usize;
        if height < 2{
            let info = self.structure_info();
            println!("{}", info);
            return
        }

        let array_height = height*2-1;
        let array_width = (2 << (height-2))*3+1 as usize;
        let mut container_raw = vec![String::from(" "); array_width*array_height];
        let mut container_base: Vec<_> = container_raw.as_mut_slice().chunks_mut(array_width).collect();
        let container: &mut [&mut [String]] = container_base.as_mut_slice();

        self.print_structure_helper(0, array_width/2, container, height);

        for i in 0..container.len() {
            let mut line = String::new();
            let mut j = 0;
            let len = container[i].len();
            loop{
                if j >= len{
                    break;
                }
                line += &container[i][j];
                if container[i][j].chars().next().unwrap() != ' '{
                    if container[i][j].len() > 4 {
                        j += 3;
                    } 
                    else{
                        j += container[i][j].len();
                    }
                }
                else{
                    j += 1;
                }
            }
            println!("{}",line);
        }
    }

    /// Helper to print_structure
    fn print_structure_helper(&self, row_index: usize, column_index: usize,
        container: &mut [&mut [String]], height: usize){
        container[row_index][column_index] = self.structure_info();
        let curr_height = (row_index+1)/2;

        if curr_height == height {
            return;
        }
        let gap = height-curr_height-1;

        if self.left().is_some(){
            container[row_index+1][column_index-gap] = String::from("/");
            let left_child = self.left().unwrap();
            left_child.borrow().print_structure_helper(row_index+2, column_index-gap*2, container, height);
        }

        if self.right().is_some(){
            container[row_index+1][column_index+gap] = String::from("\\");
            let right_child = self.right().unwrap();
            right_child.borrow().print_structure_helper(row_index+2, column_index+gap*2, container, height);
        }
    }

    /// Information in print_structure
    fn structure_info(&self)->String;

    /// Information in print_tree_as_fmt
    fn fmt_info(&self)->String;

    /// Helper of count_leaves
    fn count_leaves(&self)->u32{
        if self.is_leaf(){
            return 1;
        }
        let left_leaves = if let Some(l)=self.left(){
            l.borrow().count_leaves()
        }else{ 0 };
        let right_leaves = if let Some(r)=self.right(){
            r.borrow().count_leaves()
        }else{ 0 };
        left_leaves + right_leaves
    }
}

/// Rotation between the parent and the child
///
/// # Example
/// ```
///
/// fn left_rotate<T: Ord+Copy+Debug+Display>(root: &TreeRoot<T>){
///    let right = root.clone().unwrap().borrow().right.clone();
///   rotate(&root, &right);
///}

///fn right_rotate<T: Ord+Copy+Debug+Display>(root: &TreeRoot<T>){
///   let left = root.clone().unwrap().borrow().left.clone();
///   rotate(&root, &left);
///}
///fn left_left_rotate<T: Ord+Copy+Debug+Display>(root: &TreeRoot<T>){
///   left_rotate(&root);
///   root.clone().unwrap().borrow_mut().update_height();
///   root.clone().unwrap().borrow().parent.clone().unwrap().borrow_mut().update_height();
///}

///fn right_right_rotate<T: Ord+Copy+Debug+Display>(root: &TreeRoot<T>){
///   right_rotate(&root);
///   root.clone().unwrap().borrow_mut().update_height();
///   root.clone().unwrap().borrow().parent.clone().unwrap().borrow_mut().update_height();
///}

///fn left_right_rotate<T: Ord+Copy+Debug+Display>(root: &TreeRoot<T>){
///   let right = root.clone().unwrap().borrow().right();
///   right_right_rotate(&right);
///   left_left_rotate(&root);
///}

///fn right_left_rotate<T: Ord+Copy+Debug+Display>(root: &TreeRoot<T>){
///   let left = root.clone().unwrap().borrow().left.clone();
///   left_left_rotate(&left);
///   right_right_rotate(&root);
///}
/// ```

pub fn rotate<T: Ord+Copy+Debug+Display, N: TreeNodeTrait<T>>(parent: &Option<Rc<RefCell<N>>>,
    child: &Option<Rc<RefCell<N>>>){
    let p = parent.clone().unwrap();
    let c = child.clone().unwrap();

    let node_direction = c.borrow().get_direction_to_parent(); 

    let grad = p.borrow().parent();
    if grad.is_some(){
        let p_direct = p.borrow().get_direction_to_parent();
        match p_direct{
            Direction::Left=>{
                grad.clone().unwrap().borrow_mut().set_left(child.clone());
            },
            Direction::Right=>{
                grad.clone().unwrap().borrow_mut().set_right(child.clone());
            }
        }
    }
    match node_direction{
        Direction::Left=>{
            //rotate right
            let gc = c.borrow().right();
            p.borrow_mut().set_left(gc.clone());
            c.borrow_mut().set_right(parent.clone());
            if gc.is_some(){
                gc.unwrap().borrow_mut().set_parent(parent.clone());
            }
        },
        Direction::Right=>{
            //rotate left
            let gc = c.borrow().left();
            p.borrow_mut().set_right(gc.clone());
            c.borrow_mut().set_left(parent.clone());
            if gc.is_some(){
                gc.unwrap().borrow_mut().set_parent(parent.clone());
            }

        },
    }
    p.borrow_mut().set_parent(child.clone());
    c.borrow_mut().set_parent(grad.clone());
}

/// Helper for Tree.search()
pub fn search_node<T: Ord+Copy+Debug+Display, N: TreeNodeTrait<T>>(root: Option<Rc<RefCell<N>>>, value: T)->
    Option<Option<Rc<RefCell<N>>>>{
    if root.is_none(){
        return None;
    }

    let node = root.clone().unwrap();

    // return None, None if value is not in the tree
    let nd_val = node.borrow().value();
    match value{
        v if v < nd_val=>{
            let left = node.borrow().left();
            match left{
                None=>{return None;}
                Some(_)=>{
                    let left = node.borrow().left();
                    return search_node(left, value);
                }
            }
        },
        v if v > nd_val =>{
            let right = node.borrow().right();
            match right{
                None=>{return None;}
                Some(_)=>{
                    let right = node.borrow().right();
                    return search_node(right, value);
                }
            }
        },
        _=>{return Some(root);}
    };
}

/// Helper for Tree.insert()
pub fn search_insert_point<T: Ord+Copy+Debug+Display, N: TreeNodeTrait<T>>(root: Option<Rc<RefCell<N>>>, value: T)->
    Option<Rc<RefCell<N>>>{
    if root.is_none(){
        return None;
    }

    let node = root.clone().unwrap();

    // return None, None if value is in the tree
    let nd_val = node.borrow().value();
    match value{
        v if v < nd_val=>{
            let left = node.borrow().left();
            match left{
                None=>{return root;}
                Some(_)=>{
                    let left = node.borrow().left();
                    return search_insert_point(left, value);
                }
            }
        },
        v if v > nd_val =>{
            let right = node.borrow().right();
            match right{
                None=>{return root;}
                Some(_)=>{
                    let right = node.borrow().right();
                    return search_insert_point(right, value);
                }
            }
        },
        _=>{return None;}
    };
}