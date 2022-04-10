pub struct Solution {
    inputs: Vec<(str, str)>
}

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {

    }
}

fn main() {
    let sln = Solution {
        inputs: vec![("aa", "a"), ("aaa", "a*"), ("bab", "c")],
    };

    for input in sln.inputs {
        println!("input: {}", input);

        let output = Solution::is_match(String::from(input.0), String::from(input.1));

        println!("output: {}", output);
    }
}
