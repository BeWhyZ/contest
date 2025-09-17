use std::collections::{HashMap, VecDeque};
use std::io::{self, BufRead};

struct Solution;
// 0111 0100 xor => 0111
impl Solution {
    // 1001 1000 [0000 1000] xor => 1001 0000, 合并两个数的所有1 bit
    fn can_transform(a: Vec<i32>, b: Vec<i32>) -> bool {
        let n = a.len();

        // For each index i (1 ≤ i ≤ n), check if one of the three conditions holds:
        // 1. a[i] = b[i] (no operation needed)
        // 2. a[i] ⊕ a[i+1] = b[i] (operate on index i first, if i < n)
        // 3. a[i] ⊕ b[i+1] = b[i] (operate on index i+1 first, if i < n)

        for i in 0..n {
            let mut valid = false;

            // Condition 1: a[i] = b[i]
            if a[i] == b[i] {
                valid = true;
            }

            // Condition 2 and 3: only applicable if i < n-1 (i.e., there's an i+1)
            if i < n - 1 {
                // Condition 2: a[i] ⊕ a[i+1] = b[i]
                if a[i] ^ a[i + 1] == b[i] {
                    valid = true;
                }

                // Condition 3: a[i] ⊕ b[i+1] = b[i]
                if a[i] ^ b[i + 1] == b[i] {
                    valid = true;
                }
            }

            // If none of the conditions hold for this index, transformation is impossible
            if !valid {
                return false;
            }
        }

        true
    }

    // 图论方法：使用拓扑排序检查操作序列的合法性
    fn can_transform_graph(a: Vec<i32>, b: Vec<i32>) -> bool {
        let n = a.len();

        // 首先检查每个位置是否至少有一种合法的变换方式
        let mut operations: Vec<Vec<usize>> = vec![vec![]; n]; // operations[i] 存储位置i可能的操作方式

        for i in 0..n {
            // 方式0: 不操作 (a[i] = b[i])
            if a[i] == b[i] {
                operations[i].push(0);
            }

            // 方式1: 对位置i操作 (a[i] ⊕ a[i+1] = b[i])
            if i < n - 1 && a[i] ^ a[i + 1] == b[i] {
                operations[i].push(1);
            }

            // 方式2: 对位置i+1操作 (a[i] ⊕ b[i+1] = b[i])
            if i < n - 1 && a[i] ^ b[i + 1] == b[i] {
                operations[i].push(2);
            }

            // 如果位置i没有任何合法的变换方式，则无解
            if operations[i].is_empty() {
                return false;
            }
        }

        // 建立依赖关系图
        // 节点表示：(位置, 操作类型) -> 节点ID
        let mut node_map: HashMap<(usize, usize), usize> = HashMap::new();
        let mut node_count = 0;

        // 为每个可能的操作分配节点ID
        for i in 0..n {
            for &op in &operations[i] {
                node_map.insert((i, op), node_count);
                node_count += 1;
            }
        }

        // 构建邻接表和入度数组
        let mut graph: Vec<Vec<usize>> = vec![vec![]; node_count];
        let mut in_degree: Vec<usize> = vec![0; node_count];

        // 添加依赖边
        for i in 0..n - 1 {
            for &op_i in &operations[i] {
                for &op_i1 in &operations[i + 1] {
                    let node_i = node_map[&(i, op_i)];
                    let node_i1 = node_map[&(i + 1, op_i1)];

                    // 判断操作依赖关系
                    let need_edge = match (op_i, op_i1) {
                        (1, 2) => true,  // 先操作i，再操作i+1
                        (2, 1) => false, // 先操作i+1，再操作i（反向依赖）
                        _ => false,
                    };

                    if need_edge {
                        graph[node_i].push(node_i1);
                        in_degree[node_i1] += 1;
                    } else if matches!((op_i, op_i1), (2, 1)) {
                        graph[node_i1].push(node_i);
                        in_degree[node_i] += 1;
                    }
                }
            }
        }

        // 拓扑排序检查是否有环
        let mut queue: VecDeque<usize> = VecDeque::new();
        for i in 0..node_count {
            if in_degree[i] == 0 {
                queue.push_back(i);
            }
        }

        let mut processed = 0;
        while let Some(node) = queue.pop_front() {
            processed += 1;
            for &next in &graph[node] {
                in_degree[next] -= 1;
                if in_degree[next] == 0 {
                    queue.push_back(next);
                }
            }
        }

        // 如果所有节点都被处理，说明无环，存在合法序列
        processed == node_count
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
                "Direct method - Test case {} failed: a={:?}, b={:?}, expected={}, got={}",
                i + 1,
                a,
                b,
                expected,
                result
            );

            // 同时测试图论方法
            let result_graph = Solution::can_transform_graph(a.to_vec(), b.to_vec());
            assert_eq!(
                result_graph,
                *expected,
                "Graph method - Test case {} failed: a={:?}, b={:?}, expected={}, got={}",
                i + 1,
                a,
                b,
                expected,
                result_graph
            );

            // 确保两种方法结果一致
            assert_eq!(
                result,
                result_graph,
                "Methods inconsistent - Test case {}: a={:?}, b={:?}, direct={}, graph={}",
                i + 1,
                a,
                b,
                result,
                result_graph
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

        if Solution::can_transform_graph(a, b) {
            println!("YES");
        } else {
            println!("NO");
        }
    }
}
