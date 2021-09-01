
use algorithms::sums;

pub mod unit_tests {
    use super::*;

    #[test]
    fn it_checks_there_are_two_values_which_sum() {
        let v = vec![1, 2, 3];
        assert!(sums::has_pair_for_sum(&v, 5));

        let mut v = vec![4, 5, 6, 1, 2, 3];
        v.sort_unstable();
        assert!(sums::has_pair_for_sum(&v, 7));
        assert!(sums::has_pair_for_sum(&v, 10));
        assert!(!sums::has_pair_for_sum(&v, 12));
    }
}