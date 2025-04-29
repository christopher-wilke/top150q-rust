pub struct Solution;

impl Solution {
    pub fn simplify_path(path: String) -> String {
        let mut simplified_p: Vec<&str> = vec![];

        for v in path.split('/') {
            match v {
                "." | "" => continue,
                ".." => { simplified_p.pop(); },
                _ =>  simplified_p.push(v)
            }
        }

        format!("/{}", simplified_p.join("/"))
    }
}

fn main() {
    let path = "/n/Qz/../../ZWuLz/./R/.//".to_string();
    println!("{path}");
    let canonical_path = Solution::simplify_path(path);
    println!("{}", canonical_path);
}
