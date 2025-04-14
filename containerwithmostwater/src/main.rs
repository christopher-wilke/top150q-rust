pub struct Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut max = 0;
        let (mut left, mut right) = (0, height.len() - 1);

        while left < right {
            let h = std::cmp::min(height[left], height[right]);
            max = std::cmp::max(max, h * (right as i32 - left as i32));
            if height[left] < height[right] {
                left += 1;
            } else {
                right -= 1;
            }
        }

        max
    }
}

fn main() {
    let height = [1,8,6,2,5,4,8,3,7].to_vec();
    let solution = Solution::max_area(height);
    println!("{solution}");
}
