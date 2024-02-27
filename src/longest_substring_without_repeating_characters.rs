#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut hash = std::collections::HashMap::<u8, usize>::new();
        let mut left = 0;
        let mut right = 0;
        let mut counter = 0;
        while left < s.len() && right < s.len() {
            hash.entry(s.as_bytes()[right])
                .and_modify(|e| {
                    left = std::cmp::max(left, *e + 1);
                    *e = right
                })
                .or_insert(right);
            counter = std::cmp::max(counter, right - left + 1);
            right += 1;
        }
        counter as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(
            3,
            Solution::length_of_longest_substring("abcabcbb".to_string())
        );
    }

    #[test]
    fn case_2() {
        assert_eq!(
            1,
            Solution::length_of_longest_substring("bbbbb".to_string())
        );
    }

    #[test]
    fn case_3() {
        assert_eq!(
            3,
            Solution::length_of_longest_substring("pwwkew".to_string())
        );
    }

    #[test]
    fn case_4() {
        assert_eq!(1, Solution::length_of_longest_substring(" ".to_string()));
    }

    #[test]
    fn case_5() {
        assert_eq!(2, Solution::length_of_longest_substring("au".to_string()));
    }

    #[test]
    fn case_6() {
        assert_eq!(2, Solution::length_of_longest_substring("aab".to_string()));
    }

    #[test]
    fn case_7() {
        assert_eq!(2, Solution::length_of_longest_substring("cdd".to_string()));
    }

    #[test]
    fn case_8() {
        assert_eq!(2, Solution::length_of_longest_substring("abba".to_string()));
    }

    #[test]
    fn case_9() {
        assert_eq!(3, Solution::length_of_longest_substring("dvdf".to_string()));
    }

    #[test]
    fn case_10() {
        assert_eq!(
            5,
            Solution::length_of_longest_substring("tmmzuxt".to_string())
        );
    }
}
