#![feature(is_sorted)]

use algorithms::sort;
use rand::distributions::Uniform;
use rand::Rng;

pub mod unit_tests {
    use super::*;
    use std::cmp::Ordering;

    fn random_vec(n: usize) -> Vec<usize> {
        let mut rng = rand::thread_rng();
        let range = Uniform::new(0, n);
        (0..n).map(|_| rng.sample(&range)).collect()
    }

    #[test]
    fn it_sorts_the_array_with_insertion_sort() {
        let mut v = random_vec(1000);
        sort::insertion_sort_original(&mut v);
        assert!(v.is_sorted());
    }

    #[test]
    fn it_sorts_the_array_with_insertion_sort_binary_search() {
        let mut v = random_vec(1000);
        sort::insertion_sort(&mut v);
        assert!(v.is_sorted());
    }

    #[test]
    fn it_sorts_the_array_with_merge_sort() {
        let v = random_vec(10_000);
        let sv = sort::merge_sort(&v, 0, v.len() - 1);
        assert!(sv.is_sorted());
    }
}
