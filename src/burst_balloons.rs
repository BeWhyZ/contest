use std::cmp;

pub struct Solution {

}


impl Solution {
    pub fn max_coins(nums: Vec<i32>) -> i32 {
        // 扩展原数组来处理边界问题
        let mut points = vec![1];
        points.extend(nums);
        points.push(1);

        // 初始化dp[i][j] = 0
        let n = points.len();
        let mut dp: Vec<Vec<i32>> = vec![vec![0;n]; n];
        // 从第二个开始计算原Num的区间可能性
        for length in 2..n {
            for left in 0..=(n - length-1){
                let right = left + length;
                
                // 区间Left right中留下K的得分
                for k in (left+1)..right {
                    dp[left][right] = cmp::max(dp[left][k] + dp[k][right] + points[k] * points[left] * points[right], 
                        dp[left][right]);
                }
            }
        }
        dp[0][n-1]

    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn max_coins(){
        let nums = vec![3,1,5,8];
        assert_eq!(Solution::max_coins(nums), 167);
    }
}


