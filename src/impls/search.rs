/// binary search
pub mod bs {
    /// recursive impl
    pub fn index<T>(sv: &Vec<T>, n: &T, a: usize, b: usize) -> usize
    where
        T: std::cmp::PartialOrd,
    {
        if b - a <= 1 {
            return a;
        }

        let i = (a + b) / 2;

        match &sv[i] {
            v if v < n => index(sv, n, i, b),
            v if v > n => index(sv, n, a, i),
            _ => i,
        }
    }

    /// iterative impl
    pub fn index2<T>(sv: &Vec<T>, n: &T) -> usize
    where
        T: std::cmp::PartialOrd,
    {
        let mut a = 0;
        let mut b = sv.len() - 1;

        while b - a > 1 {
            let i = (a + b) / 2;
            let indices = match &sv[i] {
                v if v < n => (i, b),
                v if v > n => (a, i),
                _ => return i,
            };

            a = indices.0;
            b = indices.1;
        }

        a
    }

    pub fn has<T>(sv: &Vec<T>, n: &T) -> bool
    where
        T: std::cmp::PartialOrd,
    {
        &sv[index(sv, n, 0, sv.len() - 1)] == n
    }
}
