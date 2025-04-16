pub struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut longest_substr = 0;
        let mut start = 0;
        let mut last_seen = [0; 128];

        for (end, v) in s.chars().enumerate() {
            start = start.max(last_seen[v as usize]);
            longest_substr = longest_substr.max(end - start + 1);
            last_seen[v as usize] = end + 1; 
        }

        longest_substr as i32
    }
}

fn main() {
    let s = "abcabcbb".to_string();
    let solution = Solution::length_of_longest_substring(s);
    println!("{solution}");
}
