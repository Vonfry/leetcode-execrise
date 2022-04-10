pub struct Solution {
    inputs: Vec<i32>
}

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut y = x;
        let mut rev = 0;
        while y != 0 {
            if rev > i32::MAX / 10 || rev < i32::MIN / 10 {
                rev = 0;
                break;
            }
            rev = rev * 10 + y % 10;
            y /= 10;
        }

        rev
    }
}

fn main() {
    let sln = Solution {
        inputs: vec![0, -12, 121, -246],
    };

    for input in sln.inputs {
        println!("input: {}", input);

        let output = Solution::reverse(input);

        println!("output: {}", output);
    }
}
