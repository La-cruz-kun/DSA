// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl ListNode{
    fn append(&mut self, val: i32) {
        let mut pointer = self;
        while let Some(ref mut next) = pointer.next {
            pointer = next;
        }
        pointer.next = Some(Box::new(ListNode { next: None, val }));
    }
}


impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut result = l1.clone().unwrap_or(Box::new(ListNode::new(0))).val
            + l2.clone().unwrap_or(Box::new(ListNode::new(0))).val;
        let mut remainder = 0;
        if result >= 10 {
            remainder = 1;
        }
        result = result % 10;
        let mut l3 = ListNode {
            val: result,
            next: None,
        };
        let mut l1 = l1.clone().unwrap_or(Box::new(ListNode::new(0))).next;
        let mut l2 = l2.clone().unwrap_or(Box::new(ListNode::new(0))).next;
        let mut previous_remainder = remainder;
        while l1.is_some() || l2.is_some() {
            result = l1
                .clone()
                .unwrap_or(Box::new(ListNode { val: 0, next: None }))
                .val
                + l2.clone()
                    .unwrap_or(Box::new(ListNode { val: 0, next: None }))
                    .val;
            result = result + previous_remainder;
            remainder = 0;
            if result >= 10 {
                remainder = 1;
            }
            result = result % 10;
            l3.append(result);
            l1 = l1.clone().unwrap_or(Box::new(ListNode::new(0))).next;
            l2 = l2.clone().unwrap_or(Box::new(ListNode::new(0))).next;
            previous_remainder = remainder;
        }
        if previous_remainder > 0 {
            l3.append(previous_remainder);
        }
        return Some(Box::new(l3));
    }
}
