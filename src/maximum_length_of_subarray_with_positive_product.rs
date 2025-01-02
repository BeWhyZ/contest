

// 1.define the state
//  - dp_pos[i] be the length of the longest subarray ending at index i with a positive product.
//  - dp_neg[i] be the length of the longest subaaray ending at index i with a negative product.
// 2. base case
//  - nums[0] > 0 dp_pos[0] = 1,  dp_neg[0] = 0
//  - nums[0] < 0 dp_pos[0] = 0,  dp_neg[0] = 1
//  - nums[0] = 0 dp_pos[0]=dp_neg[0]=0
// 3.state transition
// - if nums[i] > 0, dp_pos[i] = dp_pos[i-1] + 1, dp_neg[i] = dp_neg[i-1]
// - if nums[i] < 0, dp_pos[i] = dp_neg[i-1] + 1, dp_neg[i] = dp_pos[i-1] + 1
// - if nums[i] = 0, dp_pos[i]=dp_neg[i]=0

use core::num;

struct Solution;


impl Solution {
    pub fn get_max_len(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut dp_pos = vec![0; n];
        let mut dp_neg = vec![0; n];

        // base case
        if nums[0] > 0 {
            dp_pos[0] = 1;
        } else if nums[0] < 0 {
            dp_neg[0] = 1;
        }

        let mut max_length =  dp_pos[0];

        // stat trans
        for i in 1..n {
            let num = nums[i];
            if num == 0 {
                dp_pos[i] = 0;
                dp_neg[i] = 0;
            } else if num > 0 {
                dp_pos[i] = dp_pos[i-1] + 1;
                dp_neg[i] = if dp_neg[i-1]>0{dp_neg[i-1]+1} else {0};
            } else {
                dp_pos[i] = if dp_neg[i-1]>0{dp_neg[i-1]+1} else {0};
                dp_neg[i] = dp_pos[i-1] + 1;
            }
            max_length = max_length.max(dp_pos[i]);
        }
        max_length

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let nums = vec![1,-2,-3,4];
        let res = Solution::get_max_len(nums);
        assert_eq!(res, 4);

        let nums = vec![-16,0,-5,2,2,-13,11,8];
        let res = Solution::get_max_len(nums);
        assert_eq!(res, 6);
        let nums = vec![-1,-2,-3,0,1];
        let res = Solution::get_max_len(nums);
        assert_eq!(res, 2);

        let nums = vec![0,1,-2,-3,-4];
        let res = Solution::get_max_len(nums);
        assert_eq!(res, 3);


        
    }
}
