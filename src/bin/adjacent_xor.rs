use std::io::{self, BufRead};

struct Solution;

impl Solution {
    fn can_transform(a: Vec<i32>, b: Vec<i32>) -> bool {
        return true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        let test_cases = [
            (vec![1, 2, 3, 4, 5], vec![3, 2, 7, 1, 5], true),
            (vec![0, 0, 1], vec![1, 0, 1], false),
            (vec![0, 0, 1], vec![0, 0, 0], false),
            (vec![0, 0, 1, 2], vec![1, 3, 3, 2], false),
            (vec![1, 1, 4, 5, 1, 4], vec![0, 5, 4, 5, 5, 4], true),
            (vec![0, 1, 2], vec![2, 3, 2], false),
            (vec![10, 10], vec![11, 10], false),
        ];

        for (i, (a, b, expected)) in test_cases.iter().enumerate() {
            let result = Solution::can_transform(a.to_vec(), b.to_vec());
            assert_eq!(
                result,
                *expected,
                "Test case {} failed: a={:?}, b={:?}, expected={}, got={}",
                i + 1,
                a,
                b,
                expected,
                result
            );
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let t: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    for _ in 0..t {
        let n: usize = loop {
            if let Some(Ok(line)) = lines.next() {
                if let Ok(num) = line.trim().parse() {
                    break num;
                }
            }
        };

        let a: Vec<i32> = loop {
            if let Some(Ok(line)) = lines.next() {
                let nums: Vec<i32> = line
                    .trim()
                    .split_whitespace()
                    .map(|x| x.parse::<i32>().unwrap())
                    .collect();
                if nums.len() != n {
                    println!("a length is not equal to n");
                    continue;
                }
                break nums;
            }
        };

        let b: Vec<i32> = loop {
            if let Some(Ok(line)) = lines.next() {
                let nums: Vec<i32> = line
                    .trim()
                    .split_whitespace()
                    .map(|x| x.parse::<i32>().unwrap())
                    .collect();
                if nums.len() != n {
                    println!("b length is not equal to n");
                    continue;
                }
                break nums;
            }
        };

        if Solution::can_transform(a, b) {
            println!("YES");
        } else {
            println!("NO");
        }
    }
}
