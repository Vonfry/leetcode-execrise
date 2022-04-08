pub struct Solution {
    inputs: Vec<Option<Box<ListNode>>>
}

impl Solution {
    pub fn merge_nodes(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        None
    }
}

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

fn main() {
    let mut sln = Solution {
        inputs: Vec::new()
    };
    let mut foo = Some(Box::new(ListNode::new(0)));
    let mut _foo = &mut foo;
    for x in [1,0,3,0,2,2,0] {
        if let Some(ref mut head) = _foo {
            head.next = Some(Box::new(ListNode::new(x)));
            _foo = &mut head.next;
        } else {
            panic!("None List init.");
        }
    }
    sln.inputs = Vec::new();
    sln.inputs.push(foo);

    for input in sln.inputs {
        print!("input:");
        let mut _input = &input;
        loop {
            match _input {
                None => break,
                Some(lst) => {
                    print!(" {}", lst.val);
                    _input = &lst.next
                }
            }
        }
        let merged = Solution::merge_nodes(input);
        let mut _merged = &merged;
        print!("output:")
        loop {
            match _merged {
                None => break,
                Some(lst) => {
                    print!(" {}", lst.val);
                    _merged = &lst.next
                }
            }
        }
    }
}
