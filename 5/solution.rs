use std::cmp::min;

pub struct Solution {
    inputs: Vec<String>
}

impl Solution {
    pub fn expand(s: &String, l: i32, r: i32) -> i32 {
        let (mut left, mut right) = (l, r);
        let s_chars: Vec<char> = s.chars().collect();
        while left >= 0 && right < s_chars.len() as i32
            && s_chars[left as usize] == s_chars[right as usize] {
            left  -= 1;
            right += 1;
        }
        (right - left - 2) / 2
    }

    pub fn longest_palindrome(s: String) -> String {
        let mut s_sharp = String::from("#");
        for c in s.chars() {
            s_sharp.push(c);
            s_sharp.push('#');
        }

        let (mut start, mut end) = (0, 0);
        let (mut right, mut j): (i32, i32)  = (-1, -1);
        let (mut min_arm, mut cur_arm): (i32, i32);
        let mut arms: Vec<usize> = Vec::new();
        for idx in 0..s_sharp.capacity() {
            let i = idx as i32;
            if right < i {
                cur_arm = Solution::expand(&s_sharp,
                                           i,
                                           i);
            } else {
                let i_sym = 2 * j - 1;
                min_arm = min(i_sym, right - i);
                cur_arm = Solution::expand(&s_sharp,
                                           i - min_arm,
                                           i + min_arm);
            }
            arms.push(cur_arm as usize);
            let new_right = cur_arm + i;
            if new_right  > right {
                j = i;
                right = new_right;
            }
            if 2 * cur_arm - 1 > end - start {
                start = i - cur_arm;
                end   = i + cur_arm;
            }
        }
        s_sharp[(start as usize)..(end as usize)].replace('#', "")
    }
}

fn main() {
    let sln = Solution {
        inputs: ["bab", "babad", "cbbd"].iter().map(|&s| s.into()).collect(),
    };

    for input in sln.inputs {
        println!("input: {}", input);

        let output = Solution::longest_palindrome(input);

        println!("output: {}", output);
    }
}
