pub struct Solution;

impl Solution {
    pub fn find_min_arrow_shots(points: Vec<Vec<i32>>) -> i32 {
        let mut points = points;
        points.sort();

        let mut p_final = vec![points[0].clone()];
        let mut pointer = 0;

        for i in 1..points.len() {
            let current_pointer = &mut p_final[pointer]; 
            if points[i][0] >= current_pointer[0] && points[i][0] <= current_pointer[1] {
                current_pointer[0] = std::cmp::max(current_pointer[0], points[i][0]);
                current_pointer[1] = std::cmp::min(current_pointer[1], points[i][1]);
            } else {
                p_final.push(points[i].clone());
                pointer += 1;
            }
        }
    
        p_final.len() as i32
    }
}

fn main() {
    // let points = [vec![10,16], vec![2,8], vec![1,6], vec![7,12]].to_vec();
    let points = [vec![1,2], vec![3,4], vec![5,6], vec![7,8]].to_vec();
    println!("{:?}", points);
    let solution = Solution::find_min_arrow_shots(points);
    println!("{:?}", solution);
}
