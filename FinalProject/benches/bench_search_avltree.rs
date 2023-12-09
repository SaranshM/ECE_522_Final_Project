use criterion::{black_box, criterion_group, criterion_main, Criterion};
use FinalProject::avltree::AVLTree; 

#[cfg(feature = "debug_print")]
macro_rules! debug_println {
    ($($args:tt)*)=>{
        println!($($args)*);
    };
}

#[cfg(not(feature = "debug_print"))]
macro_rules! debug_println {
    ($($args:tt)*)=>{};
}

fn bench_avl_tree_search(c: &mut Criterion) {
    println!("log");
    let tree_sizes = [10_000, 40_000, 70_000, 100_000, 130_000];

    for &size in &tree_sizes {
        c.bench_function(&format!("avlTree_search for {} elements", size), |b| {
            b.iter(|| {
                let mut avl_tree: AVLTree<i32> = AVLTree::<i32>::new();

                // Inserting elements
                for i in 0..size {
                    avl_tree.insert(i);
                }

                // Searching for the lowest (tree_size / 10) elements
                for i in 0..(size / 10) {
                    avl_tree.search(black_box(i));
                }
            });
        });
    }
}

criterion_group!(benches, bench_avl_tree_search);
criterion_main!(benches);
