use std::{collections::HashMap, hash::Hash};

struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map: HashMap<i64, usize> = HashMap::new();
        let mut ans: Vec<i32> = vec![];
        let target = target as i64;
        for i in 0..nums.len() {
            let req_sum = target - nums[i] as i64;
            if(map.contains_key(&req_sum)) {
                ans.push(*map.get(&req_sum).unwrap() as i32);
                ans.push(i as i32);
                return ans
            }
            map.insert(nums[i] as i64, i);
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(Solution::two_sum(vec![2,7,11,15], 9), vec![0,1]);
    }

    #[test]
    fn example_2() {
        let nums = vec![3, 2, 4];
        let target = 6;

        let result = Solution::two_sum(nums, target);

        assert_eq!(result, vec![1, 2]);
    }

    #[test]
    fn example_3() {
        let nums = vec![3, 3];
        let target = 6;

        let result = Solution::two_sum(nums, target);

        assert_eq!(result, vec![0, 1]);
    }
}