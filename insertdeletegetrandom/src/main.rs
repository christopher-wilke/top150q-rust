use std::collections::HashMap;

struct RandomizedSet {
    map: HashMap<i32, i32>,
    v: Vec<i32>
}

impl RandomizedSet {

    fn new() -> Self {
        Self {
            map: HashMap::new(),
            v: vec![]
        }
    }
    
    fn insert(&mut self, val: i32) -> bool {
        if self.map.contains_key(&val) {
            return false;
        } else {
            let pos_in_v = self.v.len() as i32;
            self.map.insert(val, pos_in_v);
            self.v.push(val);
            return true;
        }
    }
    
    fn remove(&mut self, val: i32) -> bool {
        if self.map.contains_key(&val) {
            let pos_in_v = self.map.remove(&val).unwrap();
            self.v.swap_remove(pos_in_v as usize);
            return true;
        } else {
            return false;
        }

    }
    
    fn get_random(&self) -> i32 {
        let t = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis();
        let r = t as usize % self.v.len() as usize;
        self.v[r]        
    }
}

fn main() {
    let mut obj = RandomizedSet::new();
    obj.insert(3);
    obj.insert(4);
    obj.insert(7);
    obj.remove(4);
    obj.insert(2);
    obj.insert(11);
    // let remove = obj.remove(4);
    let rnd = obj.get_random();
    println!("random: {}", rnd);
}
