pub struct Solution {
    inputs: Vec<String>
}

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        "".to_string()
    }
}

fn main() {
    let sln = Solution {
        inputs: vec!(String::from("bab"),
                     String::from("babad"),
                     String::from("cbbd")),
    };

    for input in sln.inputs {
        println!("input: {}", input);

        let output = Solution::longest_palindrome(input);

        println!("output: {}", output);
    }
}
