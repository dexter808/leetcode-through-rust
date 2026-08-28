use std::ops::Sub;
struct Solution;

impl Solution {
    pub fn erase_overlap_intervals(mut iv: Vec<Vec<i32>>) -> i32 {
        let mut ans = 0;
        iv.sort_by(|a,b| {
            a[1].cmp(&b[1])
        });

        let mut pe = iv[0][0];

        for interval in iv {
            if pe > interval[0] {
                ans += 1;
            } else {
                pe = interval[1];
            }
        }
        ans
    }
}