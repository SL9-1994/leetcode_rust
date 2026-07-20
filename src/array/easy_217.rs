use std::collections::HashSet;

pub fn easy_217() {
    let nums_1: Vec<i32> = vec![1, 2, 3, 1];
    let nums_2: Vec<i32> = vec![1, 2, 3, 4];
    let nums_3: Vec<i32> = vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2];

    println!("{}", contains_duplicate_sort(nums_1.clone()));
    println!("{}", contains_duplicate_sort(nums_2.clone()));
    println!("{}", contains_duplicate_sort(nums_3.clone()));

    println!("{}", contains_duplicate_hashset(nums_1));
    println!("{}", contains_duplicate_hashset(nums_2));
    println!("{}", contains_duplicate_hashset(nums_3));
}

// O(N log N)
pub fn contains_duplicate_sort(mut nums: Vec<i32>) -> bool {
    nums.sort();
    nums.windows(2).any(|w| w[0] == w[1])
}

// O(N)
pub fn contains_duplicate_hashset(nums: Vec<i32>) -> bool {
    let mut seen = HashSet::new();
    for num in nums {
        if !seen.insert(num) {
            return true;
        }
    }
    false
}
