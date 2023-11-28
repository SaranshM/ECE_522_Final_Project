fn main(){
    //! main
    /// It's an implementation of two binary trees, AVL tree and red-black tree.
    ///
    
    use testing::*;
    testing::main_loop();
}

mod testing{
    use std::io;
    use projectv2::prelude::AVLTree;
    use projectv2::prelude::RedBlackTree;
    use projectv2::prelude::SimpleTreeTrait;

    struct Tester{
        tree: Option<Box<dyn SimpleTreeTrait<i32>>>
    }

    const TREETYPES:[&str; 2] = ["A", "R"];

    fn menu(){
        println!("====== Tree Test ======");
        println!("0. Help");
        println!("1. Choose a tree");
        println!("2. Insert a node to the tree");
        println!("3. Delete a node from the tree");
        println!("4. Count the number of leaves in a tree");
        println!("5. Return the height of a tree");
        println!("6. Print in-order traversal of the tree");
        println!("7. Check if the tree is empty");
        println!("8. Print the tree.");
        println!("9. Print verbose information of the tree.");
        println!("10. Quit");
    }

    pub fn main_loop(){
        let mut tester = Tester{tree: None};

        menu();
        loop{
            println!("==========\nEnter 0 for help.");
            let choice = get_choice();
            match choice{
                1 =>{
                    tester = Tester::new();
                },
                10 =>{
                    println!("ByeBye!");
                    break;
                }
                0 => menu(),
                num =>{
                    if tester.tree.is_none(){
                        println!("Choose a tree before doing other operations!");
                        continue;
                    }

                    match num{
                        2 => tester.insert(),
                        3 => tester.delete(),
                        4 => tester.count_leaves(),
                        5 => tester.height(),
                        6 => tester.in_order_traverse(),
                        7 => tester.is_empty(),
                        8 => tester.print(),
                        9 => tester.print_verbose(),
                        _ => {
                            println!("Invalid choice!");
                            menu();
                            continue;
                        }
                    }
                }
            }
        }
    }

    fn get_choice()->i32{
        loop{
            println!("Input a number(0 - 10): ");
            let mut choice = String::new();
            io::stdin()
                .read_line(&mut choice)
                .expect("Failed to read line!");
            let choice: i32 = match choice.trim().parse(){
                Ok(num) => num,
                Err(_) =>{
                    println!("Please input a number!");
                    continue;
                }
            };

            if choice < 0 || choice > 10{
                println!("Please input a choice between 0 - 10");
                continue;
            }

            return choice;
        }
    }

    fn get_node()->i32{
        loop{
            println!("Input node number: ");
            let mut choice = String::new();
            io::stdin()
                .read_line(&mut choice)
                .expect("Failed to read line!");
            let choice: i32 = match choice.trim().parse(){
                Ok(num) => num,
                Err(_) =>{
                    println!("Please input a number!");
                    continue;
                }
            };
            return choice;
        }
    }

    fn get_tree_type()->String{
        loop{ 
            println!("Input tree type (R or A, R is redblacktree, A is AVL tree): ");
            let mut choice = String::new();
            io::stdin()
                .read_line(&mut choice)
                .expect("Failed to read line!");
                choice = choice.trim().to_string();
                if TREETYPES.contains(&choice.as_str()){
                    return choice;
                }
            println!("Invalid choice, valid choices are {:?}", TREETYPES);
        }
    } 

    
    impl Tester{
        fn init()->Self{
            let tmp = RedBlackTree::<i32>::new();
            let mut tester = Tester{ tree: Some(Box::new(tmp)) };
            tester.tree = None;
            tester
        }

        pub fn new()->Self{
            let choice = get_tree_type();
            println!("Choice: {}", choice);
            match choice.as_str(){
                "A"=>{
                    println!("Current Tree is AVL Tree");
                    Tester{tree: Some(Box::new(AVLTree::<i32>::new()))}
                },
                "R"=>{
                    println!("Current Tree is Red Black Tree");
                    Tester{tree: Some(Box::new(RedBlackTree::<i32>::new()))}
                },
                _=>{
                    panic!("Invalid tree type {:?}", choice);
                }
            }
        }

        fn insert(&mut self){
            let node = get_node();
            if let Some(tree) = &mut self.tree{
                if !tree.insert(node){
                    println!("The node {} already exists in the tree!", node);
                } else{
                    println!("Insert node {} successfully", node);
                }
            }
        }

        fn delete(&mut self){
            let node = get_node();
            if let Some(tree) = &mut self.tree{
                if tree.delete(node).is_none(){
                    println!("The node {} doesn't exist in the tree!", node);
                } else{
                    println!("Delete node {} successfully", node);
                }
            }
        }

        fn count_leaves(&self){
            if let Some(tree) = &self.tree{
                println!("The tree contains {} leaves", tree.count_leaves());
            }
        }

        fn height(&self){
            if let Some(tree) = &self.tree{
                println!("The tree height is {}", tree.height());
            }
        }

        fn in_order_traverse(&self){
            if let Some(tree) = &self.tree{
                let nodes = tree.in_order_traverse();
                println!("The inorder traversal of tree is {:?}", nodes);
            }
        }

        fn is_empty(&self){
            if let Some(tree) = &self.tree{
                if tree.is_empty(){
                    println!("The tree is empty");
                } else{
                    println!("The tree is not empty");
                }
            }
        }

        fn print_verbose(&self){
            if let Some(tree) = &self.tree{
                tree.print(true);
            }
        }

        fn print(&self){
            if let Some(tree) = &self.tree{
                tree.print(false);
            }
        }

    }

}