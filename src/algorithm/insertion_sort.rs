/// insertion sort; fallback to standard library's sort after (n/2)log n comparisons
pub fn insertion_sort<F, T>(slice: &mut [T], comparator: F)
where
    F: Fn(&T, &T) -> std::cmp::Ordering,
    T: Clone,
{
    let mut cmp_counter = 0;

    let n = slice.len();

    // n/2 log n
    let threshold = {
        let n = n as f64;
        let logn = n.log2();
        (n * logn / 2.) as usize
    };

    for i in 1..n {
        let target = slice[i].clone();
        let mut j = i;
        while j > 0 {
            if cmp_counter >= threshold {
                debug!("insertion sort comparison threashold reached (threshold={}, where #items={}), so data probably isn't nearly sorted; fallback to standard library's sort", threshold, n);

                slice.sort_by(comparator);
                return;
            }
            cmp_counter += 1;

            let j_minus = j - 1;

            if comparator(&slice[j_minus], &target) != std::cmp::Ordering::Greater {
                break;
            }

            // push current entry *back* by one slot, to reserve the correct slot for the target
            slice[j] = slice[j_minus].clone();
            j = j_minus;
        }

        // stuff the target into the correct slot
        slice[j] = target;
    }
    debug!(
        "insertion sort complete, #items={}, #comparisons used to sort={}.",
        slice.len(),
        cmp_counter
    );
}

#[cfg(test)]
mod insertion_sort_tests {
    use super::*;

    #[test]
    fn empty() {
        let mut a = [];
        insertion_sort(&mut a, |a: &i32, b: &i32| a.cmp(b));
        // no crash
    }

    #[test]
    fn no_duplicate() {
        let mut a = [5, 4, 2, 1, 6, 7, 23, 24];
        insertion_sort(&mut a, |a, b| a.cmp(b));

        let result = [1, 2, 4, 5, 6, 7, 23, 24];
        assert_eq!(a, result);
    }

    #[test]
    fn with_duplicate() {
        let mut a = [5, 4, 2, 2, 2, 1, 6, 1, 7, 23, 24, 2, 1];
        insertion_sort(&mut a, |a, b| a.cmp(b));

        let result = [1, 1, 1, 2, 2, 2, 2, 4, 5, 6, 7, 23, 24];
        assert_eq!(a, result);
    }
}
