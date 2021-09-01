pub mod matrix {
    use std::ops::Range;

    type Matrix = Vec<Vec<isize>>;

    fn index(m: &Matrix, idx: [&Range<usize>; 2]) -> Matrix {
        let row_start = idx[0].start;
        let row_end = idx[0].end - idx[0].start;

        let col_start = idx[1].start;
        let col_end = idx[1].end - idx[1].start;

        m.iter().skip(row_start).take(row_end)
            .map(|s| s.iter().cloned().skip(col_start).take(col_end).collect::<Vec<_>>())
            .collect::<Vec<_>>()
    }

    fn sub(m1: &Matrix, m2: &Matrix) -> Matrix {
        assert_eq!(m1.len(), m2.len());

        let mut m = Vec::new();

        for i in 0..m1.len() {
            let mut row = Vec::new();
            for j in 0..m1.len() {
                row.push(m1[i][j] - m2[i][j])
            }

            m.push(row);
        }

        m
    }

    fn add(m1: &Matrix, m2: &Matrix) -> Matrix {
        assert_eq!(m1.len(), m2.len());

        let mut m = Vec::new();

        for i in 0..m1.len() {
            let mut row = Vec::new();
            for j in 0..m1.len() {
                row.push(m1[i][j] + m2[i][j])
            }

            m.push(row);
        }

        m
    }

    pub fn mul(m1: &Matrix, m2: &Matrix) -> Matrix
    {
        assert_eq!(m1.len(), m2.len());

        let n = m1.len();

        if n == 1 {
            return vec![vec![m1[0][0] * m2[0][0]]];
        }

        let mid: usize = n / 2;
        let a = 0..mid;
        let b = mid..n;

        let m1_11 = index(m1, [&a, &a]);
        let m1_12 = index(m1, [&a, &b]);
        let m1_21 = index(m1, [&b, &a]);
        let m1_22 = index(m1, [&b, &b]);

        let m2_11 = index(m2, [&a, &a]);
        let m2_12 = index(m2, [&a, &b]);
        let m2_21 = index(m2, [&b, &a]);
        let m2_22 = index(m2, [&b, &b]);

        let s = [
            sub(&m2_12, &m2_22),
            add(&m1_11, &m1_12),
            add(&m1_21, &m1_22),
            sub(&m2_21, &m2_11),
            add(&m1_11, &m1_22),
            add(&m2_11, &m2_22),
            sub(&m1_12, &m1_22),
            add(&m2_21, &m2_22),
            sub(&m1_11, &m1_21),
            add(&m2_11, &m2_12),
        ];

        let p = Vec::from([
            mul(&m1_11, &s[0]),
            mul(&s[1], &m2_22),
            mul(&s[2], &m2_11),
            mul(&m1_22, &s[3]),
            mul(&s[4], &s[5]),
            mul(&s[6], &s[7]),
            mul(&s[8], &s[9]),
        ]);

        let c_11 = add(&add(&p[4], &p[3]), &sub(&p[5], &p[1]));
        let c_12 = add(&p[0], &p[1]);
        let c_21 = add(&p[2], &p[3]);
        let c_22 = sub(&add(&p[4], &p[0]), &add(&p[2], &p[6]));

        let mut c = Vec::new();
        for i in 0..n {
            let mut row = Vec::new();
            for j in 0..n {
                if i < n / 2 && j < n / 2 {
                    row.push(c_11[i][j]);
                } else if i < n / 2 && j >= n / 2 {
                    row.push(c_12[i][j - n / 2]);
                } else if i >= n / 2 && j < n / 2 {
                    row.push(c_21[i - n / 2][j]);
                } else if i >= n / 2 && j >= n / 2 {
                    row.push(c_22[i - n / 2][j - n / 2]);
                }
            }
            c.push(row);
        }

        c
    }
}

//
// mod old_matrix {
//     use std::borrow::Borrow;
//     use std::ops::{Add, Index, Mul, Range};
//
//     pub struct Matrix {
//         pub data: Vec<Vec<isize>>,
//     }
//
//     impl Add for Matrix {
//         type Output = Matrix;
//
//         fn add(self, other: Matrix) -> Matrix {
//             assert_eq!(self.data.len(), other.data.len());
//
//             let mut m = Matrix {
//                 data: Vec::new()
//             };
//
//             for i in 0..self.data.len() {
//                 let mut row = Vec::new();
//                 for j in 0..self.data[0].len() {
//                     row.push(self.data[i][j] + other.data[i][j])
//                 }
//
//                 m.data.push(row);
//             }
//
//             m
//         }
//     }
//
//     impl Index<[&Range<usize>; 2]> for Matrix {
//         type Output = Vec<Vec<isize>>;
//
//         fn index(&self, idx: [&Range<usize>; 2]) -> &Self::Output {
//             let row_start = idx[0].start;
//             let row_end = idx[0].end - idx[0].start;
//
//             let col_start = idx[1].start;
//             let col_end = idx[1].end - idx[1].start;
//
//             &self.data.iter().skip(row_start).take(row_end)
//                 .map(|s| s.iter().skip(col_start).take(col_end).collect::<Vec<_>>())
//                 .collect::<Vec<_>>()
//         }
//     }
//
//     impl Mul for Matrix {
//         type Output = Self;
//
//         fn mul(self, other: Self) -> Self::Output
//         {
//             let m1 = &self.data;
//             let m2 = &other.data;
//
//             assert_eq!(m1.len(), m2.len());
//
//             let n = m.len();
//             let mid: usize = n / 2;
//             let a = 0..mid;
//             let b = mid..n - 1;
//
//             let m1_11 = &self[[&a, &a]];
//             let m1_12 = &self[[&a, &b]];
//             let m1_21 = &self[[&b, &a]];
//             let m1_22 = &self[[&b, &b]];
//
//             let m2_11 = &other[[&a, &a]];
//             let m2_12 = &other[[&a, &b]];
//             let m2_21 = &other[[&b, &a]];
//             let m2_22 = &other[[&b, &b]];
//
//             let s = [
//                 m2_12 - m2_22, // sub(m2, m2, [[&a, &b], [&b, &b]])
//                 m1_11 + m1_12,
//                 m1_21 + m1_22,
//                 m2_21 - m2_11,
//                 m1_11 + m1_22,
//                 m2_11 + m2_22,
//                 m1_12 - m1_22,
//                 m2_21 + m2_22,
//                 m1_11 - m1_21,
//                 m2_11 + m2_12,
//             ];
//
//             let p = [
//                 m1_11 * s[0],
//                 s[1] * m2_22,
//                 s[2] * m2_11,
//                 m1_22 * s[3],
//                 s[4] * s[5],
//                 s[6] * s[7],
//                 s[8] * s[9],
//             ];
//
//             Matrix { data: m1_11.clone() }
//         }
//     }
// }