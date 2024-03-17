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

pub fn bench_insert_rbtree(c: &mut Criterion) {
    let tree_sizes = [10_000, 40_000, 70_000, 100_000, 130_000];

    for &size in &tree_sizes {
        c.bench_function(&format!("rbtree_insert for {} elements", size), |b| {
            b.iter(|| {
                let mut tree = RBTree::<i32>::new();
                for i in 0..size {
                    tree.insert(black_box(i));
                }
            });
        });
    }
}

criterion_group!(benches, bench_insert_rbtree); // Fix the function name here
criterion_main!(benches);
