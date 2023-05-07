#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

pub fn add_three_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>, mut rem: i32) -> Option<Box<ListNode>> {
    Some(Box::new(match (l1, l2, rem) {
        (None, None, 0) => None?,
        (None, None, val) => ListNode { next: None, val },
        (x, y, mut rem) => {
            ListNode {
                next: add_three_numbers(
                    x.and_then(|x| {rem += x.val; x.next}), 
                    y.and_then(|y| {rem += y.val; y.next}), 
                    rem / 10
                ),
                val: rem % 10,
            }
        },
    }))
}

pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    add_three_numbers(l1, l2, 0)
}