use std::collections::HashMap;

fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map = HashMap::new();
    
    for (i, &num) in nums.iter().enumerate() {
        let complement = target - num;
        
        if let Some(&j) = map.get(&complement) {
            return vec![j as i32, i as i32];
        }
        
        map.insert(num, i);
    }
    
    vec![]
}

fn main() {  
    println!("{:?}", two_sum(vec![2, 3, 5, 7], 12));
}