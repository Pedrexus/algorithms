use std::ops::Sub;
use super::sort;


pub mod sums{
    use super::*;

    pub fn has_pair_for_sum<T>(sorted_array: &Vec<T>, n: T) -> bool
    where T: Ord + Copy + Sub<Output=T>
    {
        for i in 0..sorted_array.len() // O(n)
        {
            let x = n - sorted_array[i];  // O(1)
            if sorted_array[i + 1..].binary_search(&x).is_ok()
            {
                return true;
            }
        }

        false
    }
}