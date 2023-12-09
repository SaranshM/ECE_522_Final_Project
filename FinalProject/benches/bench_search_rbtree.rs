use criterion::{black_box, criterion_group, criterion_main, Criterion};
use FinalProject::rbtree::RBTree;

#[cfg(feature = "debug_print")]
macro_rules! debug_println {
    ($($args:tt)*) => {
        println!($($args)*);
    };
}

#[cfg(not(feature = "debug_print"))]
macro_rules! debug_println {
    ($($args:tt)*) => {};
}

fn bench_search_rbtree(c: &mut Criterion) {
    println!("log");
    let tree_sizes = [10_000, 40_000, 70_000, 100_000, 130_000];

    for &size in &tree_sizes {
        c.bench_function(&format!("rbTree_search for {} elements", size), |b| {
            b.iter(|| {
                let mut rbtree: RBTree<i32> = RBTree::<i32>::new();

                // Inserting elements
                for i in 0..size {
                    rbtree.insert(black_box(i));
                }

                // Searching for the lowest (tree_size / 10) elements
                for i in 0..(size / 10) {
                    rbtree.search(black_box(i));
                }
            });
        });
    }

}

criterion_group!(benches, bench_search_rbtree);
criterion_main!(benches);
