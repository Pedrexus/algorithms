use crate::impls::matrix;

pub mod unit_tests {
    use super::*;

    #[test]
    fn it_multiplies_two_by_two_matrices() {
        let m1 = vec![
            vec![1, 2],
            vec![2, 1],
        ];

        let m2 = vec![
            vec![2, 1],
            vec![1, 2],
        ];

        let m = matrix::mul(&m1, &m2);

        let r = vec![
            vec![4, 5],
            vec![5, 4],
        ];

        assert_eq!(m, r);
    }

    #[test]
    fn it_multiplies_two_matrices_correctly() {
        let m1 = vec![
            vec![1, 2, 3, 4],
            vec![2, 1, 4, 3],
            vec![1, 3, 2, 4],
            vec![4, 3, 2, 1],
        ];

        let m2 = vec![
            vec![2, 1, 4, 3],
            vec![1, 2, 3, 4],
            vec![4, 3, 2, 1],
            vec![1, 3, 2, 4],
        ];

        let m = matrix::mul(&m1, &m2);

        assert_eq!(m[0][0], 20);
    }
}