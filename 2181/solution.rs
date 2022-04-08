pub struct Solution {
    inputs: Vec<Option<Box<ListNode>>>
}

impl Solution {
    pub fn merge_nodes(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut _fst = &head; // the first element must be 0
        let mut _snd = &head;
        let mut result = None;
        let mut _result = &mut result;
        let mut sum = 0;
        loop {
            if let Some(lst2) = _snd  {
                if lst2.val == 0 {
                    _fst = _snd;
                    if sum != 0 {
                        match _result {
                            None => {
                                *_result = Some(Box::new(ListNode::new(sum)));
                            },
                            Some(ref mut rlst)  => {
                                rlst.next = Some(Box::new(ListNode::new(sum)));
                                _result = &mut rlst.next;
                            }
                        }
                    }
                    sum = 0;
                } else {
                    sum += lst2.val;
                }
                _snd = &lst2.next;
            } else {
                break;
            }
        };

        result
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
        print!("\n");

        let merged = Solution::merge_nodes(input);
        let mut _merged = &merged;
        print!("output:");
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
