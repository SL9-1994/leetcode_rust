use std::cmp;

pub fn med_11() {
    let mut nums_1: Vec<i32> = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
    let mut nums_2: Vec<i32> = vec![1, 1];

    println!("{}", max_area(&mut nums_1));
    println!("{}", max_area(&mut nums_2));
}

fn max_area(height: &mut Vec<i32>) -> i32 {
    let mut max_water = 0;
    let mut l_idx = 0;
    let mut r_idx = height.len() - 1;

    while l_idx < r_idx {
        // 小さいほうの高さまでしか水は入らない
        let h = cmp::min(height[l_idx], height[r_idx]);
        let w = (r_idx - l_idx) as i32;
        let current_area = h * w;

        max_water = cmp::max(max_water, current_area);

        if height[l_idx] < height[r_idx] {
            l_idx += 1;
        } else {
            r_idx -= 1;
        }
    }
    max_water
}
