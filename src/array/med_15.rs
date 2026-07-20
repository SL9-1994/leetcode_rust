pub fn med_15() {
    let mut nums_1: Vec<i32> = vec![-1, 0, 1, 2, -1, -4];
    let mut nums_2: Vec<i32> = vec![0, 1, 1];
    let mut nums_3: Vec<i32> = vec![0, 0, 0];

    println!("{:?}", three_sum(&mut nums_1));
    println!("{:?}", three_sum(&mut nums_2));
    println!("{:?}", three_sum(&mut nums_3));
}

// 3つの数
pub fn three_sum(nums: &mut Vec<i32>) -> Vec<Vec<i32>> {
    let mut l_idx;
    let mut r_idx;
    let mut sum;
    let mut result: Vec<Vec<i32>> = vec![];

    let len = nums.len();

    if len < 3 {
        return result;
    }

    nums.sort();

    // nums配列の中から，重複しない3つの数を選び 0 を作る
    for i in 0..len - 2 {
        // 1つ目の要素(nums[i])の重複をスキップ
        if i > 0 && nums[i] == nums[i - 1] {
            continue;
        }

        // 最小値が0より大きければ、これ以上和が0になる組み合わせはないため抜ける
        if nums[i] > 0 {
            break;
        }

        l_idx = i + 1;
        r_idx = len - 1;

        while l_idx < r_idx {
            sum = nums[i] + nums[l_idx] + nums[r_idx];

            if sum == 0 {
                result.push(vec![nums[i], nums[l_idx], nums[r_idx]]);

                while l_idx < r_idx && nums[l_idx] == nums[l_idx + 1] {
                    l_idx += 1;
                }
                while l_idx < r_idx && nums[r_idx] == nums[r_idx - 1] {
                    r_idx -= 1;
                }

                l_idx += 1;
                r_idx -= 1;
            } else if sum < 0 {
                l_idx += 1;
            } else {
                r_idx -= 1;
            }
        }
    }

    result
}
