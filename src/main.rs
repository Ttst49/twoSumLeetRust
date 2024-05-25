



pub fn two_sum(nums: Vec<i32>, target: i32)->Vec<i32>{
    use std::collections::HashMap;

    let mut m: HashMap<i32, i32> = HashMap::new();
    for (i, v) in nums.iter().enumerate() {
        match m.get(&(target - *v)) {
            Some(&j) => return vec![i as i32, j],
            None => m.insert(*v, i as i32),
        };
    }
    vec![]
}

fn main() {
    let nums: Vec<i32> = vec![3,3];
    let target: i32 = 6;
    let result = two_sum(nums,target);
    if !result.is_empty() {
        println!("[{}][{}]",result[0],result[1])
    }else { println!("No answer") }
}
