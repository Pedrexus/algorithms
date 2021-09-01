pub mod sort {
    use std::cmp::Ordering;

    //  In a linked list, if we know the pointer to the previous node of the node to be deleted,
    // we can do deletion in O(1) time.
    pub fn insertion_sort<T>(array: &mut Vec<T>)
        where T: Ord + Copy
    {
        for i in 1..array.len() // O(n)
        {
            let idx = array[..i].binary_search(&array[i]).unwrap_or_else(|x| x); // O(log n)
            let val = array.remove(i); // O(n)
            array.insert(idx, val); // O(n)
        }
    }

    pub fn insertion_sort_original<T>(array: &mut Vec<T>)
    where T: PartialOrd + Copy
    {
        for i in 1..array.len()
        {
            let key = array[i];
            let mut j = i - 1;

            // insert array[i] into sorted array[..i]
            while array[j] > key
            {
                array[j + 1] = array[j];

                if j == 0
                {
                    break;
                }

                j -= 1;
            }

            let k = if j == 0 { 0 } else { j + 1 };
            array[k] = key;
        }
    }

    pub fn merge_sort<T>(array: &Vec<T>, a: usize, b: usize) -> Vec<T>
    where
        T: Ord + Copy,
    {
        if b - a == 1 {
            return Vec::from(&array[a..b]);
        }
        let mid = (a + b) / 2;
        merge(merge_sort(array, a, mid), merge_sort(array, mid, b))
    }

    fn merge<'a, T>(v1: Vec<T>, v2: Vec<T>) -> Vec<T>
    where
        T: Ord + Copy,
    {
        let (mut i1, mut i2) = (0usize, 0usize);

        let mut array = Vec::with_capacity(v1.len() + v2.len());
        while i1 < v1.len() && i2 < v2.len() {
            match v1[i1].cmp(&v2[i2]) {
                Ordering::Greater => {
                    array.push(v2[i2]);
                    i2 += 1;
                }
                _ => {
                    array.push(v1[i1]);
                    i1 += 1;
                }
            }
        }

        array
    }
}
