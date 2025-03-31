pub struct Solution;

impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {

        let mut current_position: usize = 3;
        let mut tank = gas[current_position];

        for i in 0..cost.len() {
            println!("current_postition: {current_position}, tank: {tank}");
            tank -= cost[current_position];
            current_position = (current_position + 1) % cost.len();
            tank += gas[current_position]; 
        }

        println!("done");
        println!("current_position: {current_position}, tank: {tank}");

        -1
    }
}

fn main() {
    let gas = [1,2,3,4,5].to_vec();
    let cost = [3,4,5,1,2].to_vec();
    println!("gas={:?}", gas);
    println!("cost={:?}", cost);
    let can_complete = Solution::can_complete_circuit(gas, cost);
    println!("{:?}", can_complete);    
}
