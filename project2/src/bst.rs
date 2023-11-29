use std::cell::RefCell;
use std::rc::Rc;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt;
use std::cmp::max;
use std::collections::VecDeque;

#[allow(non_snake_case)]

type TreeNode<T> = Rc<RefCell<Node<T>>>;
type Tree<T> = Option<TreeNode<T>>;

#[derive(Clone, PartialEq)]
pub struct Node<T: Ord+Display+Debug>{
    key: T,
    left : Tree<T>,
    right : Tree<T>,
    height : i8,
}


impl <T> Node<T>
where T: Debug+Ord+Display+Copy{
    pub fn new(key :T) -> Tree<T>{
        Some(Rc::new(RefCell::new(Node{key:key,left:None,right:None,height:1})))
    }

    pub fn height(&self) -> i8 { 
        self.height
    }
}

impl<T> fmt::Debug for Node<T>
where T: Debug+Ord+Display+Copy
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Node")
         .field("key", &self.key)
         .field("right", &self.right)
         .field("left", &self.left)
         .finish()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct AvlTree<T:Ord + Display + Debug + Copy>{
    root : Tree<T>,
    count : usize,
}
trait _Tree<T>
where T: Ord+Display+Debug+Clone+Copy{
    fn new(key:T) -> Tree<T>;
    fn height(&self,tree_node:Tree<T>) -> i8;
    fn rotate_lr(&self,tree_node:Tree<T>) -> Tree<T>;
    fn rotate_rl(&self,tree_node:Tree<T>) -> Tree<T>;
    fn update_height(&self,tree_node:Tree<T>) -> Tree<T>;
    fn do_insert(&self, root:Tree<T>,val: T) -> Tree<T>;
    fn do_delete(&self,root:Tree<T>,val:T) -> Tree<T>;
    fn balance_factor(&self,tree_node:Tree<T>) -> i8;
    fn balance_tree(&self, tree_node:Tree<T>) -> Tree<T>;
    fn rotate_left(&self,tree_node:Tree<T>) -> Tree<T>;
    fn rotate_right(&self,tree_node:Tree<T>) ->Tree<T>;
    fn find_min(&self,tree_node:Tree<T>) -> Tree<T>;
    fn find_max(&self, tree: Tree<T>) -> Tree<T>;
}

impl <T> _Tree<T> for Tree<T>
where T: Ord+Display+Debug+Clone+Copy{
    fn new(key:T) -> Tree<T> {
        Node::new(key)
    }
    fn height(&self,tree_node:Tree<T>) -> i8{
        match tree_node{
            None => 0,
            Some(node) => node.clone().borrow().height,
        }
    }
    fn update_height(&self, tree_node:Tree<T>) ->Tree<T>{
        match tree_node {
            None => tree_node,
            Some(node) => {
                let left_height = self.height(node.borrow().clone().left);
                let right_height = self.height(node.borrow().clone().right);
                node.clone().borrow_mut().height = max(left_height,right_height) + 1;
                Some(node)
            }
        }
        
    }

    fn balance_factor(&self,tree_node:Tree<T>) -> i8 {
        match tree_node{
            Some(node) =>{
                let left_height = self.height(node.borrow().clone().left.clone());
                let right_height = self.height(node.borrow().clone().right.clone());
                left_height - right_height
            }
            None => 0
        }
    }

    fn rotate_right(&self,tree_node:Tree<T>) -> Tree<T> {
        let final_tree: Tree<T>;
        match tree_node{
            None => unreachable!(),
            Some(node) =>{
                let left_node = node.borrow().left.clone();
                let left = left_node.clone();
                match left{
                    Some(left_tree) =>{
                        let left_right_tree = left_tree.borrow().right.clone();
                        final_tree = left_node.clone();
                        let new_right_tree = node.clone();
                        new_right_tree.borrow_mut().left = left_right_tree.clone();
                        let new_right_tree_updated = self.update_height(Some(new_right_tree.clone()));
                        final_tree.as_ref().unwrap().borrow_mut().right = new_right_tree_updated.clone();
                        let final_tree_updated = self.update_height(final_tree.clone());
                        final_tree_updated.clone()
                    }
                    None => unreachable!(),
                }
                
                
                
        }
    }
}

    fn rotate_left(&self,tree_node:Tree<T>) ->Tree<T> {
        let final_tree:Tree<T>; 
        match tree_node{
            None => unreachable!(),
            Some(node) =>{
                let right_node = node.borrow().right.clone();
                let right = right_node.clone();
                match right{
                    Some(right_tree) =>{
                        let right_left_tree = right_tree.borrow().left.clone();
                        final_tree = right_node.clone();
                        let new_left_tree = node.clone();
                        new_left_tree.borrow_mut().right = right_left_tree.clone();
                        let new_right_tree_updated = self.update_height(Some(new_left_tree.clone()));
                        final_tree.as_ref().unwrap().borrow_mut().left = new_right_tree_updated.clone();
                        let final_tree_updated = self.update_height(final_tree.clone());
                        final_tree_updated.clone()
                    }
                    None => unreachable!()
                }
            }
        }
    }
    fn rotate_lr(&self, tree_node:Tree<T>) -> Tree<T> {
        let rotated_tree = tree_node.clone();
        match tree_node {
            Some(root) => {
                let rotated_left_tree = self.rotate_left(root.borrow().left.clone());
                rotated_tree.as_ref().unwrap().borrow_mut().left = rotated_left_tree.clone();
                let lr_tree = self.rotate_right(rotated_tree.clone());
                lr_tree
            }
            None => unreachable!(),
        }
    }

    fn rotate_rl(&self,tree_node:Tree<T>) -> Tree<T>{
        let rotated_tree = tree_node.clone();
        match tree_node {
            Some(root) => {
                let rotated_right_tree = self.rotate_right(root.borrow().right.clone());
                rotated_tree.as_ref().unwrap().borrow_mut().right = rotated_right_tree.clone();
                let rl_tree = self.rotate_left(rotated_tree.clone());
                rl_tree
            }
            None => unreachable!(),
        }
    }

    fn do_insert(&self,tree:Tree<T>,key: T) -> Tree<T> {
        match tree {
            None => {
                let add_node = Self::new(key);
                add_node.clone()
            }
            Some(root) => {
                let clone_node = root.borrow().clone();
                let balanced_tree :Tree<T>;
                let updated_tree:Tree<T>;
                let sub_node:Tree<T>;
                if key == clone_node.key {
                    Some(root.clone())
                } 
                else if key < clone_node.key {
                    sub_node = root.borrow().left.clone();
                    let result = self.do_insert(sub_node,key);
                    let result_node = result;
                    root.borrow_mut().left = result_node;
                    let updated_tree = self.update_height(Some(root.clone()));
                    let balanced_tree = self.balance_tree(updated_tree);
                    balanced_tree.clone()
                }
                //进入右子树递归插入
                else {
                    sub_node = root.borrow().right.clone();
                    let result = self.do_insert(sub_node,key);
                    let result_node = result;
                    root.borrow_mut().right = result_node;
                    updated_tree = self.update_height(Some(root));
                    balanced_tree = self.balance_tree(updated_tree);
                    balanced_tree.clone()
                }
            }
        }
    }

    fn do_delete(&self,tree:Tree<T>,key:T) -> Tree<T>{
        let deleted_tree = tree.clone();
        let updated_tree:Tree<T>;
        let balanced_tree:Tree<T>;
        if tree.is_none(){
            return tree.clone();
        }
        else{
            let sub_node_left = tree.as_ref().unwrap().borrow().left.clone();
            let sub_node_right = tree.as_ref().unwrap().borrow().right.clone();
            if key < tree.as_ref().unwrap().borrow().key{
                deleted_tree.as_ref().unwrap().borrow_mut().left = self.do_delete(sub_node_left, key);
                updated_tree = self.update_height(deleted_tree.clone());
                balanced_tree = self.balance_tree(updated_tree);
                return balanced_tree.clone();
            }
            else if key > tree.as_ref().unwrap().borrow().key{
                deleted_tree.as_ref().unwrap().borrow_mut().right  = self.do_delete(sub_node_right, key);
                updated_tree = self.update_height(deleted_tree.clone());
                balanced_tree = self.balance_tree(updated_tree);
                return balanced_tree.clone();
            }
            else{
                if tree.as_ref().unwrap().borrow().left.is_none(){
                    let temp = tree.as_ref().unwrap().borrow().right.clone();
                    return temp.clone();
                }
                else if tree.as_ref().unwrap().borrow().right.is_none(){
                    let temp = tree.as_ref().unwrap().borrow().left.clone();
                    return temp.clone();
                }
                let temp = self.find_min(tree.as_ref().unwrap().borrow().right.clone());
                deleted_tree.as_ref().unwrap().borrow_mut().key  = temp.as_ref().unwrap().borrow().key;
                if deleted_tree.is_none(){
                    return deleted_tree.clone();
                }
                else{
                    deleted_tree.as_ref().unwrap().borrow_mut().right = self.do_delete(sub_node_right,temp.unwrap().borrow().key);
                    updated_tree = self.update_height(deleted_tree.clone());
                    balanced_tree = self.balance_tree(updated_tree);
                    return balanced_tree.clone();
                }
            }
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

    fn balance_tree(&self, tree_node:Tree<T>) -> Tree<T>{
        let balance_factor = self.balance_factor(tree_node.clone());
        let balanced_tree :Tree<T>;
        if balance_factor > 1{
            let balance_factor_left = self.balance_factor(tree_node.as_ref().unwrap().borrow().left.clone());
            if balance_factor_left >= 0{
                balanced_tree = self.rotate_right(tree_node.clone());
                return balanced_tree.clone();
            }
            else{
                return self.rotate_lr(tree_node.clone());
            }
        } 
        
        if balance_factor < -1{
            let balance_factor_right = self.balance_factor(tree_node.as_ref().unwrap().borrow().right.clone());
            if balance_factor_right <= 0{
                return self.rotate_left(tree_node.clone());
            }
            else{
                return self.rotate_rl(tree_node.clone());
            }
        }
        tree_node
    }
}

impl <T> AvlTree<T>
where T: Ord+Display+Debug+Clone+Copy{
    pub fn new() -> Self{
        AvlTree { root: None ,count: 0}
    }

    pub fn count(&self) -> usize {
        return self.count;
    }

    pub fn is_empty(&self) -> bool{
        if self.root.is_none(){
            return true;
        }
        else {
            return false;
        }
    }

    pub fn height(&self) -> i8{
        self.root.height(self.root.clone())
    }

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

    pub fn insert(&mut self,key:T){
        let root_node = self.root.clone();
        let res_tree = self.root.do_insert(root_node,key);
        self.root = res_tree;
        self.count += 1;
    }

    
    pub fn delete(&mut self,key:T){
        let root_node = self.root.clone();
        let res_tree = self.root.do_delete(root_node.clone(),key);
        self.root = res_tree;
        self.count -= 1;
    }

    pub fn search(&self, key: T) -> Tree<T> {
        let dummy = Node::<T>::new(key).unwrap().borrow().clone();
        self.search_node(&self.root, &dummy)
    }
    
    pub fn min(&self) -> Tree<T> {
        self.root.find_min(self.root.clone())
    }

    pub fn max(&self) -> Tree<T> {
        self.root.find_max(self.root.clone())
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
    pub fn print_tree(&self) {
        if self.root.is_none() {
            println!("None");
            return;
        };

        fn pretty_print<T: Ord+Display+Debug+Clone>(node: TreeNode<T>, buffer: &mut String, prefix: &mut String, child_prefix: &String) {
            let node_height = node.borrow().clone().height;
            prefix.push_str(&("(".to_string() + &node_height.to_string() + &")".to_string()));
            buffer.push_str(&prefix);
            buffer.push_str(&node.borrow().clone().key.to_string());
            buffer.push_str(&"\n".to_string());
            for child in [node.borrow().clone().right.clone(), node.borrow().clone().left.clone()] {
                if child.is_some() {
                    if child.as_ref().unwrap().borrow().clone().left.is_some() || child.as_ref().unwrap().borrow().clone().right.is_some() {
                        let mut new_prefix = child_prefix.clone();
                        let mut new_child_prefix = child_prefix.clone();
                        new_prefix.push_str(&"├── ");
                        new_child_prefix.push_str(&"│   ");
                        pretty_print(child.as_ref().unwrap().clone(), buffer, &mut new_prefix, &new_child_prefix);
                    } else {
                        let mut new_prefix = child_prefix.clone();
                        let mut new_child_prefix = child_prefix.clone();
                        new_prefix.push_str(&"└── ");
                        new_child_prefix.push_str(&"    ");
                        pretty_print(child.as_ref().unwrap().clone(), buffer, &mut new_prefix, &new_child_prefix);
                    }
                }
            }
        }

        let node = self.root.as_ref().unwrap().clone();
        let mut buffer = String::new();
        pretty_print(node, &mut buffer, &mut "".to_string(), &"".to_string());
        println!("{}", buffer);
    }   
}

impl<T> fmt::Display for AvlTree<T>
where T: Debug+Ord+Display+Copy
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("AvlTree")
         .field("root", &self.root)
         .finish()
    }
}

#[test]
pub fn create_empty_avltree() {
    // type T must be specified if theres no other node insertions
    let avltree: AvlTree<u32> = AvlTree::new();
    assert!(avltree.root.is_none());
}

#[test]
pub fn insert_into_avltree_1() {
    let mut x = AvlTree::new();
    assert_eq!(0,x.count);
    x.insert(3);
    assert_eq!(x.root.as_ref().unwrap().borrow().key, 3);
    assert_eq!(x.count,1);
    assert_eq!(x.height(),1);
    x.insert(2);
    assert_eq!(x.root.as_ref().unwrap().borrow().left.as_ref().unwrap().borrow().key, 2);
    assert_eq!(x.count, 2);
    assert_eq!(x.height(),2);
}

#[test]
pub fn insert_into_avltree_2() {
    //  ll rotation
    let mut x = AvlTree::new();
    assert_eq!(0,x.count);
    x.insert(3);
    assert_eq!(x.root.as_ref().unwrap().borrow().key, 3);
    assert_eq!(x.count,1);
    assert_eq!(x.height(),1);
    x.insert(2);
    assert_eq!(x.root.as_ref().unwrap().borrow().left.as_ref().unwrap().borrow().key, 2);
    assert_eq!(x.count, 2);
    assert_eq!(x.height(),2);
    x.insert(1);
    assert_eq!(x.height(),2);
    assert_eq!(x.root.as_ref().unwrap().borrow().key,2);
    assert_eq!(x.root.as_ref().unwrap().borrow().left.as_ref().unwrap().borrow().key,1);
    assert_eq!(x.root.as_ref().unwrap().borrow().right.as_ref().unwrap().borrow().key,3);
    assert_eq!(x.count, 3);
}

#[test]
pub fn insert_into_rbtree_3() {
    // lr rotation
    let mut x = AvlTree::new();
    assert_eq!(0,x.count);
    x.insert(4);
    assert_eq!(x.root.as_ref().unwrap().borrow().key, 4);
    assert_eq!(x.count,1);
    assert_eq!(x.height(),1);
    x.insert(2);
    assert_eq!(x.root.as_ref().unwrap().borrow().left.as_ref().unwrap().borrow().key, 2);
    assert_eq!(x.count, 2);
    assert_eq!(x.height(),2);
    x.insert(3);
    assert_eq!(x.root.as_ref().unwrap().borrow().key,3);
    assert_eq!(x.root.as_ref().unwrap().borrow().left.as_ref().unwrap().borrow().key,2);
    assert_eq!(x.root.as_ref().unwrap().borrow().right.as_ref().unwrap().borrow().key,4);
    assert_eq!(x.count, 3);
    assert_eq!(x.height(),2);
}

#[test]
pub fn insert_into_rbtree_4() {
    // rr rotaion
    let mut x = AvlTree::new();
    assert_eq!(0,x.count);
    x.insert(4);
    assert_eq!(x.root.as_ref().unwrap().borrow().key, 4);
    assert_eq!(x.count,1);
    assert_eq!(x.height(),1);
    x.insert(5);
    assert_eq!(x.root.as_ref().unwrap().borrow().right.as_ref().unwrap().borrow().key, 5);
    assert_eq!(x.count, 2);
    assert_eq!(x.height(),2);
    x.insert(6);
    assert_eq!(x.root.as_ref().unwrap().borrow().key,5);
    assert_eq!(x.root.as_ref().unwrap().borrow().left.as_ref().unwrap().borrow().key,4);
    assert_eq!(x.root.as_ref().unwrap().borrow().right.as_ref().unwrap().borrow().key,6);
    assert_eq!(x.count, 3);
    assert_eq!(x.height(),2);
}

#[test]
pub fn insert_into_rbtree_5() {
    // rl rotation
    let mut x = AvlTree::new();
    assert_eq!(0,x.count);
    x.insert(4);
    assert_eq!(x.root.as_ref().unwrap().borrow().key, 4);
    assert_eq!(x.count,1);
    assert_eq!(x.height(),1);
    x.insert(6);
    assert_eq!(x.root.as_ref().unwrap().borrow().right.as_ref().unwrap().borrow().key, 6);
    assert_eq!(x.count, 2);
    assert_eq!(x.height(),2);
    x.insert(5);
    assert_eq!(x.root.as_ref().unwrap().borrow().key,5);
    assert_eq!(x.root.as_ref().unwrap().borrow().left.as_ref().unwrap().borrow().key,4);
    assert_eq!(x.root.as_ref().unwrap().borrow().right.as_ref().unwrap().borrow().key,6);
    assert_eq!(x.count, 3);
    assert_eq!(x.height(),2);
}
 
#[test]
pub fn search_1() {
    let mut x = AvlTree::new();
    x.insert(9);
    x.insert(8);
    x.insert(12);
    x.insert(3);

    let y = x.search(8);
    assert_eq!(8,y.as_ref().unwrap().borrow().clone().key);
    let z = x.search(81);
    assert!(z.is_none());
}

#[test]
pub fn test_delete_1() {
    let mut x = AvlTree::new();
    x.insert(12);
    x.insert(8);
    x.insert(15);
    assert_eq!(x.height(),2);
    x.delete(12);
    assert_eq!(x.height(),2);
    assert_eq!(x.root.as_ref().unwrap().borrow().key,15);
    assert_eq!(x.root.as_ref().unwrap().borrow().left.as_ref().unwrap().borrow().key,8);
}

#[test]
pub fn insert_test_1() {
    let mut x = AvlTree::new();
    x.insert(15);
    x.insert(11);
    x.insert(19);
    x.insert(8);
    x.insert(13);
    x.insert(16);
    x.insert(23);
    x.insert(12);
    x.insert(14);
    assert_eq!(x.root.as_ref().unwrap().borrow().key,15);
    assert_eq!(x.height(),4);

    let x_right = x.root.as_ref().unwrap().borrow().clone().right.clone();
    assert_eq!(x_right.as_ref().unwrap().borrow().key,19);
    assert_eq!(x_right.height(x_right.clone()),2);

    let x_left = x.root.as_ref().unwrap().borrow().clone().left.clone();
    assert_eq!(x_left.as_ref().unwrap().borrow().key,11);
    assert_eq!(x_left.height(x_left.clone()),3);

    let x_right_right = x_right.as_ref().unwrap().borrow().clone().right.clone();
    assert_eq!(x_right_right.as_ref().unwrap().borrow().key,23);

    let x_right_left = x_right.as_ref().unwrap().borrow().clone().left.clone();
    assert_eq!(x_right_left.as_ref().unwrap().borrow().key,16);

    let x_left_left = x_left.as_ref().unwrap().borrow().clone().left.clone();
    assert_eq!(x_left_left.as_ref().unwrap().borrow().key,8);

    let x_left_right = x_left.as_ref().unwrap().borrow().clone().right.clone();
    assert_eq!(x_left_right.as_ref().unwrap().borrow().key,13);

    let x_left_right_right = x_left_right.as_ref().unwrap().borrow().clone().right.clone();
    assert_eq!(x_left_right_right.as_ref().unwrap().borrow().key,14);
    let x_left_right_left = x_left_right.as_ref().unwrap().borrow().clone().left.clone();
    assert_eq!(x_left_right_left.as_ref().unwrap().borrow().key,12);
}

#[test]
pub fn delete_test_1() {
    let mut x = AvlTree::new();
    x.insert(15);
    x.insert(11);
    x.insert(19);
    x.insert(8);
    x.insert(13);
    x.insert(16);
    x.insert(23);
    x.insert(12);
    x.insert(14);
    x.delete(13);
    assert_eq!(x.root.as_ref().unwrap().borrow().key,15);

    let x_right = x.root.as_ref().unwrap().borrow().clone().right.clone();
    assert_eq!(x_right.as_ref().unwrap().borrow().key,19);

    let x_left = x.root.as_ref().unwrap().borrow().clone().left.clone();
    assert_eq!(x_left.as_ref().unwrap().borrow().key,11);

    let x_right_right = x_right.as_ref().unwrap().borrow().clone().right.clone();
    assert_eq!(x_right_right.as_ref().unwrap().borrow().key,23);

    let x_right_left = x_right.as_ref().unwrap().borrow().clone().left.clone();
    assert_eq!(x_right_left.as_ref().unwrap().borrow().key,16);

    let x_left_left = x_left.as_ref().unwrap().borrow().clone().left.clone();
    assert_eq!(x_left_left.as_ref().unwrap().borrow().key,8);

    let x_left_right = x_left.as_ref().unwrap().borrow().clone().right.clone();
    assert_eq!(x_left_right.as_ref().unwrap().borrow().key,14);

    let x_left_right_left = x_left_right.as_ref().unwrap().borrow().clone().left.clone();
    assert_eq!(x_left_right_left.as_ref().unwrap().borrow().key,12);
}

#[test]
pub fn insert_test_2() {
    let mut x = AvlTree::new();
    x.insert(10);
    x.insert(5);
    x.insert(30);
    x.insert(2);
    x.insert(9);
    x.insert(25);
    x.insert(40);
    x.insert(38);
    assert_eq!(x.root.as_ref().unwrap().borrow().key,10);

    let x_right = x.root.as_ref().unwrap().borrow().clone().right.clone();
    assert_eq!(x_right.as_ref().unwrap().borrow().key,30);

    let x_left = x.root.as_ref().unwrap().borrow().clone().left.clone();
    assert_eq!(x_left.as_ref().unwrap().borrow().key,5);

    let x_right_right = x_right.as_ref().unwrap().borrow().clone().right.clone();
    assert_eq!(x_right_right.as_ref().unwrap().borrow().key,40);

    let x_right_left = x_right.as_ref().unwrap().borrow().clone().left.clone();
    assert_eq!(x_right_left.as_ref().unwrap().borrow().key,25);

    let x_right_right_left = x_right_right.as_ref().unwrap().borrow().clone().left.clone();
    assert_eq!(x_right_right_left.as_ref().unwrap().borrow().key,38);

    let x_left_left = x_left.as_ref().unwrap().borrow().clone().left.clone();
    assert_eq!(x_left_left.as_ref().unwrap().borrow().key,2);

    let x_left_right = x_left.as_ref().unwrap().borrow().clone().right.clone();
    assert_eq!(x_left_right.as_ref().unwrap().borrow().key,9);
}

#[test]
pub fn delete_test_2() {
    let mut x = AvlTree::new();
    x.insert(10);
    x.insert(5);
    x.insert(30);
    x.insert(2);
    x.insert(9);
    x.insert(25);
    x.insert(40);
    x.insert(38);
    x.delete(30);
    assert_eq!(x.root.as_ref().unwrap().borrow().key,10);

    let x_right = x.root.as_ref().unwrap().borrow().clone().right.clone();
    assert_eq!(x_right.as_ref().unwrap().borrow().key,38);

    let x_left = x.root.as_ref().unwrap().borrow().clone().left.clone();
    assert_eq!(x_left.as_ref().unwrap().borrow().key,5);

    let x_right_right = x_right.as_ref().unwrap().borrow().clone().right.clone();
    assert_eq!(x_right_right.as_ref().unwrap().borrow().key,40);

    let x_right_left = x_right.as_ref().unwrap().borrow().clone().left.clone();
    assert_eq!(x_right_left.as_ref().unwrap().borrow().key,25);

    let x_left_left = x_left.as_ref().unwrap().borrow().clone().left.clone();
    assert_eq!(x_left_left.as_ref().unwrap().borrow().key,2);

    let x_left_right = x_left.as_ref().unwrap().borrow().clone().right.clone();
    assert_eq!(x_left_right.as_ref().unwrap().borrow().key,9);
}

#[test]
pub fn insert_test_3() {
    let mut x = AvlTree::new();
    x.insert(10);
    x.insert(5);
    x.insert(20);
    x.insert(15);
    x.insert(30);
    assert_eq!(x.root.as_ref().unwrap().borrow().key,10);

    let x_right = x.root.as_ref().unwrap().borrow().clone().right.clone();
    assert_eq!(x_right.as_ref().unwrap().borrow().key,20);

    let x_left = x.root.as_ref().unwrap().borrow().clone().left.clone();
    assert_eq!(x_left.as_ref().unwrap().borrow().key,5);

    let x_right_right = x_right.as_ref().unwrap().borrow().clone().right.clone();
    assert_eq!(x_right_right.as_ref().unwrap().borrow().key,30);

    let x_right_left = x_right.as_ref().unwrap().borrow().clone().left.clone();
    assert_eq!(x_right_left.as_ref().unwrap().borrow().key,15);
}

#[test]
pub fn delete_test_3() {
    let mut x = AvlTree::new();
    x.insert(10);
    x.insert(5);
    x.insert(20);
    x.insert(15);
    x.insert(30);
    assert_eq!(x.root.as_ref().unwrap().borrow().key,10);
    
    let x_right = x.root.as_ref().unwrap().borrow().clone().right.clone();
    assert_eq!(x_right.as_ref().unwrap().borrow().key,20);
 

    let x_left = x.root.as_ref().unwrap().borrow().clone().left.clone();
    assert_eq!(x_left.as_ref().unwrap().borrow().key,5);
    
    let x_right_right = x_right.as_ref().unwrap().borrow().clone().right.clone();
    assert_eq!(x_right_right.as_ref().unwrap().borrow().key,30);
}

#[test]
pub fn insert_test_4() {
    let mut x = AvlTree::new();
    x.insert(10);
    x.insert(5);
    x.insert(20);
    x.insert(1);
    x.insert(7);
    x.insert(15);
    x.insert(30);
    assert_eq!(x.root.as_ref().unwrap().borrow().key,10);
    

    let x_right = x.root.as_ref().unwrap().borrow().clone().right.clone();
    assert_eq!(x_right.as_ref().unwrap().borrow().key,20);


    let x_left = x.root.as_ref().unwrap().borrow().clone().left.clone();
    assert_eq!(x_left.as_ref().unwrap().borrow().key,5);


    let x_right_right = x_right.as_ref().unwrap().borrow().clone().right.clone();
    assert_eq!(x_right_right.as_ref().unwrap().borrow().key,30);


    let x_right_left = x_right.as_ref().unwrap().borrow().clone().left.clone();
    assert_eq!(x_right_left.as_ref().unwrap().borrow().key,15);


    let x_left_left = x_left.as_ref().unwrap().borrow().clone().left.clone();
    assert_eq!(x_left_left.as_ref().unwrap().borrow().key,1);


    let x_left_right = x_left.as_ref().unwrap().borrow().clone().right.clone();
    assert_eq!(x_left_right.as_ref().unwrap().borrow().key,7);

}

#[test]
pub fn delete_test_4() {
    let mut x = AvlTree::new();
    x.insert(10);
    x.insert(5);
    x.insert(20);
    x.insert(1);
    x.insert(7);
    x.insert(15);
    x.insert(30);
    x.delete(15);
    assert_eq!(x.root.as_ref().unwrap().borrow().key,10);

    let x_right = x.root.as_ref().unwrap().borrow().clone().right.clone();
    assert_eq!(x_right.as_ref().unwrap().borrow().key,20);

    let x_left = x.root.as_ref().unwrap().borrow().clone().left.clone();
    assert_eq!(x_left.as_ref().unwrap().borrow().key,5);

    let x_right_right = x_right.as_ref().unwrap().borrow().clone().right.clone();
    assert_eq!(x_right_right.as_ref().unwrap().borrow().key,30);

    let x_left_left = x_left.as_ref().unwrap().borrow().clone().left.clone();
    assert_eq!(x_left_left.as_ref().unwrap().borrow().key,1);

    let x_left_right = x_left.as_ref().unwrap().borrow().clone().right.clone();
    assert_eq!(x_left_right.as_ref().unwrap().borrow().key,7);
}

#[test]
pub fn insert_test_5() {
    let mut x = AvlTree::new();
    x.insert(10);
    x.insert(5);
    x.insert(20);
    x.insert(1);
    x.insert(7);
    x.insert(15);
    x.insert(30);
    x.insert(25);
    x.insert(40);
    assert_eq!(x.root.as_ref().unwrap().borrow().key,10);

    let x_right = x.root.as_ref().unwrap().borrow().clone().right.clone();
    assert_eq!(x_right.as_ref().unwrap().borrow().key,20);

    let x_left = x.root.as_ref().unwrap().borrow().clone().left.clone();
    assert_eq!(x_left.as_ref().unwrap().borrow().key,5);

    let x_right_right = x_right.as_ref().unwrap().borrow().clone().right.clone();
    assert_eq!(x_right_right.as_ref().unwrap().borrow().key,30);

    let x_right_left = x_right.as_ref().unwrap().borrow().clone().left.clone();
    assert_eq!(x_right_left.as_ref().unwrap().borrow().key,15);

    let x_right_right_left = x_right_right.as_ref().unwrap().borrow().clone().left.clone();
    assert_eq!(x_right_right_left.as_ref().unwrap().borrow().key,25);

    let x_right_right_right = x_right_right.as_ref().unwrap().borrow().clone().right.clone();
    assert_eq!(x_right_right_right.as_ref().unwrap().borrow().key,40);

    let x_left_left = x_left.as_ref().unwrap().borrow().clone().left.clone();
    assert_eq!(x_left_left.as_ref().unwrap().borrow().key,1);


    let x_left_right = x_left.as_ref().unwrap().borrow().clone().right.clone();
    assert_eq!(x_left_right.as_ref().unwrap().borrow().key,7);

}

#[test]
pub fn delete_test_5() {
    let mut x = AvlTree::new();
    x.insert(10);
    x.insert(5);
    x.insert(20);
    x.insert(1);
    x.insert(7);
    x.insert(15);
    x.insert(30);
    x.insert(25);
    x.insert(40);
    x.delete(15);
    assert_eq!(x.root.as_ref().unwrap().borrow().key,10);


    let x_right = x.root.as_ref().unwrap().borrow().clone().right.clone();
    assert_eq!(x_right.as_ref().unwrap().borrow().key,30);


    let x_left = x.root.as_ref().unwrap().borrow().clone().left.clone();
    assert_eq!(x_left.as_ref().unwrap().borrow().key,5);


    let x_right_right = x_right.as_ref().unwrap().borrow().clone().right.clone();
    assert_eq!(x_right_right.as_ref().unwrap().borrow().key,40);


    let x_right_left = x_right.as_ref().unwrap().borrow().clone().left.clone();
    assert_eq!(x_right_left.as_ref().unwrap().borrow().key,20);


    let x_right_left_right = x_right_left.as_ref().unwrap().borrow().clone().right.clone();
    assert_eq!(x_right_left_right.as_ref().unwrap().borrow().key,25);


    let x_left_left = x_left.as_ref().unwrap().borrow().clone().left.clone();
    assert_eq!(x_left_left.as_ref().unwrap().borrow().key,1);


    let x_left_right = x_left.as_ref().unwrap().borrow().clone().right.clone();
    assert_eq!(x_left_right.as_ref().unwrap().borrow().key,7);
}

#[test]
pub fn insert_test_6() {
    let mut x = AvlTree::new();
    x.insert(1);
    x.insert(5);
    x.insert(7);
    x.insert(10);
    x.insert(20);
    x.insert(25);
    x.insert(28);
    x.insert(30);
    x.insert(40);
    assert_eq!(x.root.as_ref().unwrap().borrow().key,10);

    let x_right = x.root.as_ref().unwrap().borrow().clone().right.clone();
    assert_eq!(x_right.as_ref().unwrap().borrow().key,25);

    let x_left = x.root.as_ref().unwrap().borrow().clone().left.clone();
    assert_eq!(x_left.as_ref().unwrap().borrow().key,5);

    let x_right_right = x_right.as_ref().unwrap().borrow().clone().right.clone();
    assert_eq!(x_right_right.as_ref().unwrap().borrow().key,30);

    let x_right_left = x_right.as_ref().unwrap().borrow().clone().left.clone();
    assert_eq!(x_right_left.as_ref().unwrap().borrow().key,20);

    let x_right_right_left = x_right_right.as_ref().unwrap().borrow().clone().left.clone();
    assert_eq!(x_right_right_left.as_ref().unwrap().borrow().key,28);

    let x_right_right_right = x_right_right.as_ref().unwrap().borrow().clone().right.clone();
    assert_eq!(x_right_right_right.as_ref().unwrap().borrow().key,40);

    let x_left_left = x_left.as_ref().unwrap().borrow().clone().left.clone();
    assert_eq!(x_left_left.as_ref().unwrap().borrow().key,1);

    let x_left_right = x_left.as_ref().unwrap().borrow().clone().right.clone();
    assert_eq!(x_left_right.as_ref().unwrap().borrow().key,7);
}

#[test]
pub fn delete_test_6() {
    let mut x = AvlTree::new();
    x.insert(1);
    x.insert(5);
    x.insert(7);
    x.insert(10);
    x.insert(20);
    x.insert(25);
    x.insert(28);
    x.insert(30);
    x.insert(40);
    x.delete(1);
    assert_eq!(x.root.as_ref().unwrap().borrow().key,10);

    let x_right = x.root.as_ref().unwrap().borrow().clone().right.clone();
    assert_eq!(x_right.as_ref().unwrap().borrow().key,25);

    let x_left = x.root.as_ref().unwrap().borrow().clone().left.clone();
    assert_eq!(x_left.as_ref().unwrap().borrow().key,5);

    let x_right_right = x_right.as_ref().unwrap().borrow().clone().right.clone();
    assert_eq!(x_right_right.as_ref().unwrap().borrow().key,30);

    let x_right_left = x_right.as_ref().unwrap().borrow().clone().left.clone();
    assert_eq!(x_right_left.as_ref().unwrap().borrow().key,20);

    let x_right_right_left = x_right_right.as_ref().unwrap().borrow().clone().left.clone();
    assert_eq!(x_right_right_left.as_ref().unwrap().borrow().key,28);

    let x_right_right_right = x_right_right.as_ref().unwrap().borrow().clone().right.clone();
    assert_eq!(x_right_right_right.as_ref().unwrap().borrow().key,40);

    let x_left_right = x_left.as_ref().unwrap().borrow().clone().right.clone();
    assert_eq!(x_left_right.as_ref().unwrap().borrow().key,7);
}

#[test]
pub fn test_letters() {
    let mut x = AvlTree::new();
    x.insert("a");
    x.insert("b");
    x.insert("c");
    x.insert("p");
    x.insert("m");
    x.delete("c");

    assert_eq!(x.root.as_ref().unwrap().borrow().key,"b");

    let x_right = x.root.as_ref().unwrap().borrow().clone().right.clone();
    assert_eq!(x_right.as_ref().unwrap().borrow().key,"m");

    let x_left = x.root.as_ref().unwrap().borrow().clone().left.clone();
    assert_eq!(x_left.as_ref().unwrap().borrow().key,"a");

    let x_right_right = x_right.as_ref().unwrap().borrow().clone().right.clone();
    assert_eq!(x_right_right.as_ref().unwrap().borrow().key, "p");
}

#[test]
pub fn test_min_max_1() {
    let mut a = AvlTree::new();
    a.insert(455);
    a.insert(32);
    a.insert(4);
    a.insert(9);
    a.insert(12);
    a.insert(1);
    assert_eq!(a.min().as_ref().unwrap().borrow().key, 1);
    assert_eq!(a.max().as_ref().unwrap().borrow().key, 455);
}

#[test]
pub fn test_min_max_2() {
    let mut a = AvlTree::new();
    a.insert("a");
    a.insert("f");
    a.insert("d");
    a.insert("g");
    a.insert("u");
    a.insert("c");
    assert_eq!(a.min().as_ref().unwrap().borrow().key, "a");
    assert_eq!(a.max().as_ref().unwrap().borrow().key, "u");
}