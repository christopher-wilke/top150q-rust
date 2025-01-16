fn binary_search(arr: &[i32], target: i32) -> Option<usize> {
    let mut low = 0;
    let mut high = arr.len() - 1;

    while low <= high {
        let mid = high - low / 2;

        match arr[mid].cmp(&target) {
            std::cmp::Ordering::Less => {
                low = mid + 1;
            }
            std::cmp::Ordering::Greater => {
                // if mid == 0 { break; } // Avoid underflow for usize
                high = mid - 1;
            }
            std::cmp::Ordering::Equal => {
                return Some(mid);
            }
        }
    }
    None
}

fn main() {
    let arr = [1, 3, 5, 7, 9, 11, 13];
    let target = 13;

    match binary_search(&arr, target) {
        Some(index) => println!("Found {} at index {}", target, index),
        None => println!("{} not found in the array", target),
    }
}
