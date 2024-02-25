#[allow(dead_code)]
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

#[allow(dead_code)]
impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }

    pub fn as_vec(&self) -> Vec<i32> {
        let mut vec = Vec::new();
        let mut current = self;
        loop {
            vec.push(current.val);
            match &current.next {
                Some(node) => current = node,
                _ => return vec,
            };
        }
    }

    pub fn from_vec(value: Vec<i32>) -> Option<Box<Self>> {
        let mut head = None;
        let mut current = &mut head;
        for e in value {
            let node = Box::new(ListNode::new(e));
            current.replace(node);
            if let Some(node) = current {
                current = &mut node.next;
            }
        }
        head
    }
}
