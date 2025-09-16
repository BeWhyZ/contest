struct Solution;

impl Solution {
    // 优雅的数学解法 - O(n log n) 时间复杂度
    pub fn unjust_binary_life_mathematical(a: String, b: String, n: usize) -> i64 {
        let a: Vec<i64> = a.chars().map(|c| c.to_digit(10).unwrap() as i64).collect();
        let b: Vec<i64> = b.chars().map(|c| c.to_digit(10).unwrap() as i64).collect();

        let first_term = (n * n * (n + 1) / 2) as i64;

        // 求解 ∑|prea(x) - preb(y)|
        // prea a[1..x]中 0的个数减去1的个数
        let mut prea = vec![0i64; n + 1];
        let mut preb = vec![0i64; n + 1];
        for i in 1..=n {
            prea[i] = prea[i - 1] + if a[i - 1] == 0 { 1 } else { -1 };
            preb[i] = preb[i - 1] + if b[i - 1] == 1 { 1 } else { -1 };
        }
        let sum_abs_diff = Self::calculate_sum_abs_differences(&mut prea[1..], &mut preb[1..]);

        return first_term - sum_abs_diff / 2;
    }

    // 优化后的版本 - 真正的 O(n log n)
    fn calculate_sum_abs_differences(prea: &mut [i64], preb: &mut [i64]) -> i64 {
        // 求解 ∑|prea(x) - preb(y)|
        // 优化详情 若B[i] 已知升序，则可以利用二分查找优化查找，并利用前缀和

        prea.sort_unstable();
        preb.sort_unstable();

        let mut preb_prefix = vec![0i64; preb.len() + 1];
        for i in 1..=preb.len() {
            preb_prefix[i] = preb_prefix[i - 1] + preb[i - 1];
        }

        let mut tot = 0i64;
        let mut j = 0usize;
        for &mut prea_val in prea {
            while j < preb.len() && preb[j] < prea_val {
                j += 1;
            }

            tot += j as i64 * prea_val - preb_prefix[j];
            tot +=
                (preb_prefix.last().unwrap() - preb_prefix[j]) - (preb.len() - j) as i64 * prea_val;
        }
        return tot;
    }
}

fn main() {
    // 接收命令行输入的数字 n 表示一共有n个测试

    use std::io::{self, BufRead};

    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // 读取测试用例数量
    let t: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    for _ in 0..t {
        // 读取字符串长度
        let n: usize = loop {
            if let Some(Ok(line)) = lines.next() {
                if let Ok(num) = line.trim().parse() {
                    break num;
                }
            }
        };

        // 读取字符串a
        let a = loop {
            if let Some(Ok(line)) = lines.next() {
                let s = line.trim();
                if s.len() == n {
                    break s.to_string();
                }
            }
        };

        // 读取字符串b
        let b = loop {
            if let Some(Ok(line)) = lines.next() {
                let s = line.trim();
                if s.len() == n {
                    break s.to_string();
                }
            }
        };

        let result = Solution::unjust_binary_life_mathematical(a, b, n);

        // 最终结果
        println!("{}", result);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let a = "11".to_string();
        let b = "00".to_string();
        let n = a.len();
        let result = Solution::unjust_binary_life_mathematical(a, b, n);
        assert_eq!(result, 5);
    }
}
