pub struct Solution {
    inputs: Vec<(&'static str, &'static str)>
}

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let s: Vec<char> = s.chars().collect();
        let p: Vec<char> = p.chars().collect();
        let s_len = s.len();
        let p_len = p.len();

        let match_c = |i, j| -> bool {
            i != 0 && (p[j - 1] == '.' || s[i - 1] == p[j - 1])
        };

        let mut dp = vec![vec![false; p_len + 1]; s_len + 1];
        dp[0][0] = true;
        (0..=s_len).for_each(|i| {
            (1..=p_len).for_each(|j| {
                dp[i][j] = if p[j - 1] != '*' {
                    match_c(i, j) && dp[i - 1][j - 1]
                } else {
                    match_c(i, j - 1) && dp[i - 1][j] || dp[i][j - 2]
                }
            })
        });
        dp[s_len][p_len]
    }
}

fn main() {
    let sln = Solution {
        inputs: vec![("aa", "a"), ("aaa", "a*"), ("bab", "c")],
    };

    for input in sln.inputs {
        println!("input: {}, {}", input.0, input.1);

        let output = Solution::is_match(String::from(input.0), String::from(input.1));

        println!("output: {}", output);
    }
}
