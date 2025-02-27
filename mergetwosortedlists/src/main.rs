// Definition for singly-linked list.
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

pub struct Solution;

impl Solution {
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {

       
        match (list1, list2) {
            (None, None) => {
                None   
            },
            (Some(list), None) => {
                Some(list)
            },
            (None, Some(list)) => {
                Some(list)
            },
            (Some(mut list1), Some(mut list2)) => {
                if list1.val < list2.val {
                    list1.next = Self::merge_two_lists(list1.next, Some(list2));
                    Some(list1)
                } else 
                    list2.next = Self::merge_two_lists(Some(list1), list2.next);
                    Some(list2)
                }
            }
        }
    }
}

pub fn main() {
    let list1 = Some(
        Box::new(ListNode {
            val: 1,
            next: Some(Box::new(
                ListNode {
                    val: 2,
                    next: Some(Box::new(
                        ListNode { 
                            val: 4, 
                            next: None 
                        }
                    ))
                }
            ))
        })
    );
    
    let list2 = Some(
        Box::new(ListNode {
            val: 1,
            next: Some(Box::new(
                ListNode {
                    val: 3,
                    next: Some(Box::new(
                        ListNode { 
                            val: 4, 
                            next: None 
                        }
                    ))
                }
            ))
        })
    );    

    let m = Solution::merge_two_lists(list1, list2);
    // println!("{:?}", m);
}
