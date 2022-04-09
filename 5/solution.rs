use std::cmp::min;

pub struct Solution {
    inputs: Vec<String>
}

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let mut s_sharp = String::from("^#");
        for c in s.chars() {
            s_sharp.push(c);
            s_sharp.push('#');
        }
        s_sharp.push('$'); // add these due to range in loop

        let (mut right, mut j) = (0, 1);
        let mut arms: Vec<usize> = Vec::new();
        let s_chars: Vec<char> = s_sharp.chars().collect();
        let char_len = s_chars.len();
        arms.resize(char_len, 0);
        for i in 1..(char_len - 1) {
            let i_sym = if 2 * j > i { 2 * j - i } else { 0 };

            arms[i] = if right > i { min(right - i, arms[i_sym]) } else { 0 };

            loop {
                let mut l = i - 1 - arms[i];
                let mut r = i + 1 + arms[i];
                if l >= 0 && r < char_len && s_chars[l] == s_chars[r] {
                    arms[i] += 1;
                } else {
                    break;
                }
            }

            if i + arms[i] > right {
                j = i;
                right = i + arms[i];
            }
        }

        match arms.iter().enumerate().max_by(|&x, &y| {
            x.1.cmp(y.1)
        }) {
            Some((center_idx, &max_len)) => {
                s.chars().skip((center_idx - 1 - max_len) >> 1).take(max_len).collect()
            },
            None => String::new(),
        }
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
