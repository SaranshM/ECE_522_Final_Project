use project2::rbtree::RBTree;
use project2::avltree::AVLTree;
use std::env;
use std::io;
use std::io::Write;
use std::thread;
use std::time::Duration;


fn main() {
    println!("Welcome!");
    println!("");
    loop {
        println!("Select the tree:");
        println!("1. Red Black Tree");
        println!("2. AVL Tree");

        print!("Enter your choice (1 or 2): ");
        io::stdout().flush().unwrap(); // Flush to make sure 'Enter your choice' is printed before input

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1" => {
                println!("");
                println!("-----------------------");
                println!("");
                println!("You selected Red Black Tree!");
                println!("");
                run_rbtree();
                break;
            },
            "2" => {
                println!("");
                println!("-----------------------");
                println!("");
                println!("You selected AVL Tree!");
                println!("");
                run_avltree();
                break;
            },
            _ => {
                println!("Invalid input. Please select either 1 or 2.");
            }
        }
    }
}

fn run_rbtree() {
    let mut tree: RBTree<u32> = RBTree::<u32>::new();
    let mut option: u32;
    loop {
        print_options();
        println!("Enter choice of operation: ");
        option = handle_user_input();
        println!("");
        let num: u32;
        match option {
            0 => {
                println!("Thank you for using our application!");
                break;
            }
            1 => {
                // insert node
                println!("Enter element to INSERT: ");
                num = handle_user_input();
                tree.insert(num);
                println!("");
                println!("STATUS: Element inserted successfully!");
                println!("");
                println!("-----------------------");
                println!("");
            },
            2 => {
                // delete node
                println!("Enter element to DELETE: ");
                num = handle_user_input();
                tree.delete(num);
                println!("");
                println!("STATUS: Element deleted successfully!");
                println!("");
                println!("-----------------------");
                println!("");
            },
            3 => {
                // count leaves
                println!("There are {} leaf node(s) in the tree.", tree.leaves());
                println!("");
                println!("-----------------------");
                println!("");
            },
            4 => {
                // count leaves
                println!("The height of the tree is {}", tree.height());
                println!("");
                println!("-----------------------");
                println!("");
            },
            5 => {
                // in-order traversal
                println!("Inorder Traversal ->");
                println!("");
                tree.print_inorder();
                println!("");
                println!("-----------------------");
                println!("");
            },
            6 => {
                // is tree empty
                println!("Is tree empty? {}", tree.is_empty());
                println!("");
                println!("-----------------------");
                println!("");
            },
            7 => {
                // print the tree
                //println!("{:#?}",tree);
                println!("Tree structure ->");
                println!("");
                tree.print_tree();
                println!("");
                println!("-----------------------");
                println!("");
            },
            8 => {
                // number of nodes
                println!("There are {} node(s) in the tree", tree.count());
                println!("");
                println!("-----------------------");
                println!("");
            },
            9 => {
                // search for a node
                println!("Enter key of node you would like to search for: ");
                num = handle_user_input();
                println!("Is key present? {:#?}", tree.search_element(num));
                println!("");
                println!("-----------------------");
                println!("");
            },
            10 => {
                // pre-order traversal
                println!("Preorder Traversal ->");
                println!("");
                tree.print_preorder();
                println!("");
                println!("-----------------------");
                println!("");
            },
            11 => {
                // level-order traversal
                println!("Level order Traversal ->");
                println!("");
                tree.print_levelorder();
                println!("");
                println!("-----------------------");
                println!("");
            },
            12 => {
                // debug print
                println!("{:#?}", tree);
            },
            _ => {break;}
        }
    };
}

fn run_avltree() {
    let mut tree: AVLTree<u32> = AVLTree::<u32>::new();
    let mut option: u32;
    loop {
        print_options();
        option = handle_user_input();
        let num: u32;
        match option {
            0 => {
                println!("Thank you for using our application!");
                break;
            }
            1 => {
                // insert node
                println!("Enter element to INSERT: ");
                num = handle_user_input();
                tree.insert(num);
            },
            2 => {
                // delete node
                println!("Enter element to DELETE: ");
                num = handle_user_input();
                tree.delete(num);
            },
            3 => {
                // count leaves
                println!("There are {} leaf node(s) in the tree.", tree.count_leaves());
            },
            4 => {
                // return height
                // height function needs to be public
                println!("The height of the tree is {}", tree.height());
            },
            5 => {
                // in-order traversal
                println!("In-order Traversal: {:?}", tree.inorder_traversal());
            },
            6 => {
                // is tree empty
                println!("Is tree empty? {}", tree.is_empty());
            },
            7 => {
                // print the tree
                tree.print_tree();
            },
            8 => {
                // number of nodes
                println!("There are {} node(s) in the tree", tree.count());
            },
            9 => {
                // search for a node
                println!("Enter key of node you would like to search for: ");
                num = handle_user_input();
                println!("Is key present? {:#?}", tree.search(num));
                println!("");
                println!("-----------------------");
                println!("");
            },
            10 => {
                // pre-order traversal
                tree.print_preorder();
            },
            11 => {
                // level-order traversal
                tree.print_levelorder();
            },
            12 => {
                // debug print
                println!("{:#?}", tree);
            },
            _ => {break;}
        }
    };
}

fn print_options() {
    thread::sleep(Duration::from_secs(1));
    println!("Select one of the following operations: ");
    println!("0. To exit application");
    println!("1. To insert node");
    println!("2. To delete node");
    println!("3. To count number of leaves");
    println!("4. To return the height");
    println!("5. To print in-order traversal");
    println!("6. To check if the tree is empty");
    println!("7. To print the tree");
    println!("8. To return number of nodes");
    println!("9. To search for a node");
    println!("10. To print pre-order traversal");
    println!("11. To print level-order traversal");
    println!("12. To debug print");
    println!(" ");
}

fn handle_user_input() -> u32 {
    let mut selection = String::new();
    io::stdin()
        .read_line(&mut selection)
        .expect("Failed to read line");
    
    let num;
    match selection.trim().parse() {
        Ok(x) => {num = x;}
        Err(_) => {
            panic!("Invalid input. Please try again.")
        }
    }
    num
}