#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut hash = std::collections::HashMap::<i32, i32>::new();
        for (i, num) in nums.iter().enumerate() {
            let pointer = target - num;
            match hash.get(&pointer) {
                Some(j) => return vec![i as i32, *j],
                None => {
                    hash.insert(*num, i as i32);
                }
            };
        }
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(vec![1, 0], Solution::two_sum(vec![2, 7, 11, 15], 9));
    }

    #[test]
    fn case_2() {
        assert_eq!(vec![2, 1], Solution::two_sum(vec![3, 2, 4], 6));
    }

    #[test]
    fn case_3() {
        assert_eq!(vec![1, 0], Solution::two_sum(vec![3, 3], 6));
    }
}
