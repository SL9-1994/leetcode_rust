pub fn easy_27() {
    let mut nums_1: Vec<i32> = vec![3, 2, 2, 3];
    let mut nums_2: Vec<i32> = vec![0, 1, 2, 2, 3, 0, 4, 2];

    println!("{}", remove_element(&mut nums_1, 3));
    println!("{}", remove_element(&mut nums_2, 2));
}

fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    nums.retain(|&n| n != val);
    nums.len() as i32
}
