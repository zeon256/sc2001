use sc2001::{insertion_sort::InsertionSort, merge_sort::MergeSort};

/// # Type parameter
///
/// * `S` - Threshold to swap to insertion sort
pub struct InsertionMergeSort;

impl InsertionMergeSort {
    pub fn sort_key_cmp<T: Ord + Copy>(buf: &mut [T], s: usize, key_cmp: &mut u64) {
        let sz = buf.len();

        if sz <= s {
            InsertionSort::sort_key_cmp(buf, key_cmp);
            return;
        }

        let (l_buf, r_buf) = buf.split_at_mut(buf.len() / 2);

        Self::sort_key_cmp(l_buf, s, key_cmp);
        Self::sort_key_cmp(r_buf, s, key_cmp);
        MergeSort::merge_key_cmp(l_buf, r_buf, key_cmp);
    }

    pub fn sort<T: Ord + Copy>(buf: &mut [T], s: usize) {
        let sz = buf.len();

        if sz <= s {
            InsertionSort::sort(buf);
            return;
        }

        let (l_buf, r_buf) = buf.split_at_mut(buf.len() / 2);

        Self::sort(l_buf, s);
        Self::sort(r_buf, s);
        MergeSort::merge(l_buf, r_buf);
    }
}

#[cfg(test)]
mod test {
    use crate::insertion_merge::InsertionMergeSort;

    use sc2001::test_utils::{assert_sorted, gen_random_array};

    #[test]
    fn test_insertion_merge_sort_random() {
        let mut data = gen_random_array::<10000, _>(None);
        InsertionMergeSort::sort(&mut data, 15);
        assert_sorted(&data);
    }

    #[test]
    #[ignore]
    fn key_cmp_vs_s() {
        use std::{cmp::Reverse, fs::File, io::Write};

        use nanorand::{Rng, WyRand};

        let mut rng = WyRand::new();
        let data = gen_random_array::<1000000, _>(420);
        let data_1 = gen_random_array::<1000000, _>(123);
        let data_2 = gen_random_array::<1000000, _>(888);

        // nearly sorted
        let mut data_3 = gen_random_array::<1000000, _>(6969);
        data_3.sort_unstable();

        for _ in 0..100 {
            data_3.swap(
                rng.generate_range(0..1000000),
                rng.generate_range(0..1000000),
            );
        }

        // sorted backwards
        let mut data_4 = gen_random_array::<1000000, _>(8907);
        data_4.sort_by_key(|w| Reverse(*w));

        for i in 1..1000000 {
            assert!(data_4[i - 1] >= data_4[i]);
        }

        let mut f = File::create("key_cmp_2.csv").unwrap();
        let mut string =
            String::from("s,key_cmp,key_cmp1,key_cmp2,key_cmp3(nearly_sorted),key_cmp4(rsort)\n");

        for s in 3..=512 {
            let (mut key_cmp_0, mut key_cmp_1, mut key_cmp_2, mut key_cmp_3, mut key_cmp_4) =
                (0, 0, 0, 0, 0);
            let mut data_0 = data.clone();
            let mut data_1 = data_1.clone();
            let mut data_2 = data_2.clone();
            let mut data_3 = data_3.clone();
            let mut data_4 = data_4.clone();

            InsertionMergeSort::sort_key_cmp(&mut data_0, s, &mut key_cmp_0);
            InsertionMergeSort::sort_key_cmp(&mut data_1, s, &mut key_cmp_1);
            InsertionMergeSort::sort_key_cmp(&mut data_2, s, &mut key_cmp_2);
            InsertionMergeSort::sort_key_cmp(&mut data_3, s, &mut key_cmp_3);
            InsertionMergeSort::sort_key_cmp(&mut data_4, s, &mut key_cmp_4);
            println!(
                "s: {}, key_cmp_0: {}, key_cmp_1: {}, key_cmp_2: {}, key_cmp_3: {}, key_cmp_4: {}",
                s, key_cmp_0, key_cmp_1, key_cmp_2, key_cmp_3, key_cmp_4
            );

            string.push_str(&format!(
                "{},{},{},{},{},{}\n",
                s, key_cmp_0, key_cmp_1, key_cmp_2, key_cmp_3, key_cmp_4
            ));
        }

        f.write(string.as_bytes()).unwrap();
    }
}
