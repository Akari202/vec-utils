#![feature(iter_intersperse)]
use std::time::Instant;
use itertools::Itertools;

fn main() {
    let it = (0..100000).map(|_| {"hey"});

    let iter = it.clone();
    let before_collect = Instant::now();
    let _s1 = iter.collect::<Vec<&str>>().join("\n");
    let after_collect = Instant::now();

    let iter = it.clone();
    let before_fold = Instant::now();
    let s2 = iter.fold(String::new(), |a, b| a + b + "\n");
    let _s2 = s2.trim_end();
    let after_fold = Instant::now();

    let iter = it.clone();
    let before_std_inter = Instant::now();
    let _s3 = Iterator::intersperse(iter, "\n").collect::<String>();
    let after_std_inter = Instant::now();

    let iter = it.clone();
    let before_iter_inter = Instant::now();
    let _s4 = Itertools::intersperse(iter, "\n").collect::<String>();
    let after_iter_inter = Instant::now();

    println!("Collect Time: {:?}", after_collect - before_collect);
    println!("Fold Time: {:?}", after_fold - before_fold);
    println!("Stdlib Intersperse Time: {:?}", after_std_inter - before_std_inter);
    println!("Itertools Intersperse Time: {:?}", after_iter_inter - before_iter_inter);
}
