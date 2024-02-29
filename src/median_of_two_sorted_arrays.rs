#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn find_median_sorted_arrays(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> f64 {
        if nums1.len() > nums2.len() {
            (nums1, nums2) = (nums2, nums1)
        }
        let (m, n) = (nums1.len(), nums2.len());
        let (mut left, mut right) = (0, m);
        while left <= right {
            let partition_a = (left + right) / 2;
            let partition_b = (m + n + 1) / 2 - partition_a;
            let max_left_a = if partition_a == 0 {
                i32::MIN
            } else {
                *nums1.get(partition_a - 1).unwrap()
            };
            let min_right_a = if partition_a == m {
                i32::MAX
            } else {
                *nums1.get(partition_a).unwrap()
            };
            let max_left_b = if partition_b == 0 {
                i32::MIN
            } else {
                *nums2.get(partition_b - 1).unwrap()
            };
            let min_right_b = if partition_b == n {
                i32::MAX
            } else {
                *nums2.get(partition_b).unwrap()
            };
            if max_left_a <= min_right_b && max_left_b <= min_right_a {
                return if (m + n) % 2 == 0 {
                    (std::cmp::max(max_left_a, max_left_b)
                        + std::cmp::min(min_right_a, min_right_b)) as f64
                        / 2.
                } else {
                    std::cmp::max(max_left_a, max_left_b) as f64
                };
            } else if max_left_a > max_left_b {
                right = partition_a - 1;
            } else {
                left = partition_a + 1;
            }
        }
        0.
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(2., Solution::find_median_sorted_arrays(vec![1, 3], vec![2]));
    }

    #[test]
    fn case_2() {
        assert_eq!(
            2.5,
            Solution::find_median_sorted_arrays(vec![1, 2], vec![3, 4])
        );
    }
}
