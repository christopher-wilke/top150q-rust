pub struct Solution;

pub struct CharMatrix(Vec<Vec<char>>);

impl std::fmt::Debug for CharMatrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in &self.0 {
            let ff = format!("{:?}\n", i);
            write!(f, "{}", ff)? 
        }
        write!(f, "")
    }
}

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows <= 1 {
            return s;
        }

        let mut row: usize = 0;
        let mut column: usize = 0;

        let row_len = s.chars().count() / num_rows as usize;
        let mut result = CharMatrix(vec![vec![' '; row_len + num_rows as usize]; num_rows as usize]);
        let mut zig_zag = false;

        for c in s.chars() {
            result.0[row][column] = c;

            if !zig_zag {
                row += 1;
                if row == num_rows as usize {
                    row -= 2;
                    column += 1;
                    if num_rows > 2 {
                        zig_zag = true;    
                    }
                }                
            } else {
                row -= 1;
                column += 1;
                if row == 0 {
                    zig_zag = false;
                }
            }
        }

        let mut res = String::new();

        for row in result.0 {
            for c in row {
                if c != ' ' {
                    res.push_str(String::from(c).as_str());
                }
            }
        }

        res
    }
}

fn main() {
    let s = "PAYPALISHIRING".to_string();
    let res = Solution::convert(s, 3);
    println!("{res}");
}
