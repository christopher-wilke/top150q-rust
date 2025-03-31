use std::collections::VecDeque;

pub struct Solution;

const ONES : [&str;10] = ["", "I", "II", "III", "IV", "V", "VI", "VII", "VIII", "IX"];
const TENS : [&str;10] = ["", "X", "XX", "XXX", "XL", "L", "LX", "LXX", "LXXX", "XC"];
const CENT : [&str;10] = ["", "C", "CC", "CCC", "CD", "D", "DC", "DCC", "DCCC", "CM"];
const MILS : [&str;4]  = ["", "M", "MM", "MMM"];

impl Solution 
{
    pub fn int_to_roman(num: i32) -> String 
    {
        // Given that the number of outcomes is small, a brute force
		// substituion for each power of ten is a viable solution...
		format!("{}{}{}{}", MILS[(num / 1000 % 10) as usize],
                            CENT[(num / 100  % 10) as usize],
                            TENS[(num / 10   % 10) as usize],
                            ONES[(num        % 10) as usize])
    }
}

fn main() {
    let num = 123874;
    println!("{num}");
    println!("{}", (num / 1000)%10);
}
