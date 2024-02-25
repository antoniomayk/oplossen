use crate::collections::ListNode;

#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn add_two_numbers(
        mut l1: Option<Box<ListNode>>,
        mut l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut head = Option::<Box<ListNode>>::None;
        let mut tail = &mut head;
        let mut carry = 0;
        while l1.is_some() || l2.is_some() || carry != 0 {
            let sum: i32 = match (l1, l2) {
                (Some(n1), Some(n2)) => {
                    (l1, l2) = (n1.next, n2.next);
                    n1.val + n2.val
                }
                (Some(n1), None) => {
                    (l1, l2) = (n1.next, None);
                    n1.val
                }
                (None, Some(n2)) => {
                    (l1, l2) = (None, n2.next);
                    n2.val
                }
                _ => {
                    (l1, l2) = (None, None);
                    0
                }
            } + carry;
            match tail {
                Some(node) => {
                    node.next = Option::from(Box::new(ListNode::new(sum % 10)));
                    tail = &mut node.next;
                }
                _ => {
                    tail.replace(Box::new(ListNode::new(sum % 10)));
                }
            }
            carry = sum / 10;
        }
        head
    }
}

#[cfg(test)]
mod tests {
    use crate::collections::ListNode;

    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(
            ListNode::from_vec(vec![7, 0, 8]),
            Solution::add_two_numbers(
                ListNode::from_vec(vec![2, 4, 3]),
                ListNode::from_vec(vec![5, 6, 4]),
            )
        );
    }

    #[test]
    fn case_2() {
        assert_eq!(
            ListNode::from_vec(vec![0]),
            Solution::add_two_numbers(ListNode::from_vec(vec![0]), ListNode::from_vec(vec![0]))
        );
    }

    #[test]
    fn case_3() {
        assert_eq!(
            ListNode::from_vec(vec![8, 9, 9, 9, 0, 0, 0, 1]),
            Solution::add_two_numbers(
                ListNode::from_vec(vec![9, 9, 9, 9, 9, 9, 9]),
                ListNode::from_vec(vec![9, 9, 9, 9]),
            )
        );
    }
}
