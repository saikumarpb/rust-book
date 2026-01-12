use std::collections::HashMap;

pub fn median(nums: &mut Vec<i32>) -> i32 {
    nums.sort();
    nums[(nums.len() - 1) / 2]
}

pub fn mode(nums: &mut Vec<i32>) -> (i32, i32) {
    let mut result: i32 = nums.get(0).copied().unwrap_or(0);
    let mut max_count = 0;

    let mut map: HashMap<&i32, i32> = HashMap::new();

    for num in nums {
        let count = map.entry(num).or_insert(0);
        *count += 1;

        if *count >= max_count {
            max_count = *count;
            result = *num;
        }
    }

    (result, max_count)
}
