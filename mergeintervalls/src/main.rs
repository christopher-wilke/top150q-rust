pub struct Solution;

impl Solution {
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut v = vec![];
        let mut intervals = intervals; 
        intervals.sort_by(|a, b| a[0].cmp(&b[0]));
        let (mut start, mut end): (i32, i32) = (intervals[0][0], intervals[0][1]);

        for i in 1..intervals.len() {
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
    let mut intervals = [
        [1,4].to_vec(),
        [2,3].to_vec(),
        // [8,10].to_vec(),
        // [9,18].to_vec()
    ].to_vec();
    intervals.sort_by(|a, b| a[0].cmp(&b[0]));
    let solution = Solution::merge(intervals);
    println!("{:?}", solution);
}
