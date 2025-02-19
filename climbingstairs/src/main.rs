use std::collections::HashMap;

pub struct Solution;

impl Solution {

    // Time: O(2^n)
    // Space: O(n) - Stack Calls
    pub fn climb_stairs(n: i32) -> i32 {
        if n == 0 || n == 1 {
            return 1;
        }

        return Self::climb_stairs(n-1) + Self::climb_stairs(n-2);
    }

    // Optimized (iterative), bottom-up
    // Time Complexity: O(n)
    // Space Complexity: O(1)
    pub fn climb_stairs_iterative(n: i32) -> i32 {
        if n == 1 {
            return 1;
        }

        let (mut first, mut second) = (1, 1);

        for _ in 2..=n {
            let tmp = first+second;
            first = second;
            second = tmp;
        }

        return second;
    }

    pub fn climb_stairs_memo(n: i32, memo: &mut HashMap<i32, i32>) -> i32 {
        if n == 0 || n == 1 {
            return 1;
        }

        if let Some(&result) = memo.get(&n) {
            return result;
        }

        let result = Self::climb_stairs_memo(n-1, memo) + Self::climb_stairs_memo(n-2, memo);
        return result;
    }
    
}

fn main() {
    let n = 5;

    let mut hm = HashMap::new();
    let solution = Solution::climb_stairs_memo(n, &mut hm);
    println!("{solution}");
}
