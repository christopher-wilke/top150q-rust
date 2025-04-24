pub struct Solution;

impl Solution {
    pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let mut v = vec![];
        let intervals = intervals;
        let (mut start, mut end): (i32, i32) = (intervals[0][0], intervals[0][1]);

        for i in 0..intervals.len() {
            if new_interval[0] <= intervals[i][1] {
                if new_interval[1] > intervals[i][1] {
                    end = new_interval[1];
                } 
            }
            
            if intervals[i][0] <= end {
                // we skip if we have a lower item
                if intervals[i][1] > end {
                    end = intervals[i][1];    
                }
            } else {
                v.push(vec![start, end]);
                start = intervals[i][0];
                end = intervals[i][1];
            }
        }
        
        v.push(vec![start, end]);    
        v
    }
}

fn main() {
    let intervals = [[1,3].to_vec(), [6,9].to_vec()].to_vec();
    let new_interval = [2,5].to_vec();
    let solution = Solution::insert(intervals, new_interval);
    println!("{:?}", solution);
}
