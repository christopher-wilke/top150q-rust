struct MinStack {
    // (value, lowest_seen_value)
    v: Vec<(i32, i32)>
}

impl MinStack {

    fn new() -> Self {
        Self {
            v: vec![]
        }        
    }
    
    fn push(&mut self, val: i32) {
        if self.v.len() == 0 {
            self.v.push((val, val));
        } else {
            let last_lowest = &self.v[self.v.len() - 1].1;
            let lowest = last_lowest.min(&val);
            self.v.push((val, *lowest));
        }
    }
    
    fn pop(&mut self) {
        self.v.pop();
    }
    
    fn top(&self) -> i32 {
        self.v[self.v.len() - 1].0
    }
    
    fn get_min(&self) -> i32 {
        self.v[self.v.len() - 1].1
    }
}

fn main() {
    let mut obj = MinStack::new();
    obj.push(-2);
    obj.push(0);
    obj.push(-3);
    let mut min = obj.get_min();
    println!("{min}");
    obj.pop();
    let top = obj.top();
    println!("top={top}");
    min = obj.get_min();
    println!("{min}");
}
