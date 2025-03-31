pub struct Solution;

impl Solution {
    pub fn reverse_words(s: String) -> String {
        let split: Vec<&str> = s.split_whitespace().collect();
        println!("{split:?}");
        let mut answer = vec![];
        let mut i = split.len() as i32 - 1;

        while i >= 0 {
            answer.push(split[i as usize]);
            i -= 1;
        }

        answer.join(" ")
    }
}

fn main() {
    let s = "  the  sky is blue  ".to_string();
    Solution::reverse_words(s);
}
