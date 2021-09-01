#![feature(is_sorted)]

use crate::impls::bs;

pub mod unit_tests {
    use super::*;

    #[test]
    fn it_finds_the_index_in_a_small_sorted_array_1() {
        let v = vec![1, 2, 3, 4, 5];
        assert_eq!(bs::index(&v, &4, 0, v.len() - 1), 3);
    }

    #[test]
    fn it_finds_the_index_in_a_large_sorted_array_1() {
        let v: Vec<u64> = (0..10_000).map(|x| x * x).collect();
        assert_eq!(bs::index(&v, &(99 * 99), 0, v.len() - 1), 99);
    }

    #[test]
    fn it_finds_the_index_in_a_small_sorted_array_2() {
        let v = vec![1, 2, 3, 4, 5];
        assert_eq!(bs::index2(&v, &4), 3);
    }

    #[test]
    fn it_finds_the_index_in_a_large_sorted_array_2() {
        let v: Vec<u64> = (0..10_000).map(|x| x * x).collect();
        assert_eq!(bs::index2(&v, &(99 * 99)), 99);
    }

    #[test]
    fn it_checks_array_has_item() {
        let v: Vec<u64> = (0..10_000).map(|x| x * x).collect();
        assert!(bs::has(&v, &(99 * 99)));
        assert!(!bs::has(&v, &99));
    }
}
