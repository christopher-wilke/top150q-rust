pub struct Solution;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        if strs.len() <= 1 {
            return vec![strs];
        }

        let mut hm: std::collections::HashMap<Vec<char>, usize> = std::collections::HashMap::new();
        let mut solution: Vec<Vec<String>> = vec![];

        for v in strs {
            let mut chars = v.chars().collect::<Vec<char>>();
            chars.sort();
            if let Some(idx) = hm.get(&chars) {
                solution[*idx].push(v);
            } else {
                solution.push(vec![v]);
                hm.insert(chars, solution.len() - 1);
            }
        }

        solution
    }
}

fn main() {
    let strs = [
        "eat".to_owned(),
        "tea".to_owned(),
        "tan".to_owned(),
        "ate".to_owned(),
        "nat".to_owned(),
        "bat".to_owned()
    ].to_vec();
    let solution = Solution::group_anagrams(strs);
    println!("{:?}", solution);
}
