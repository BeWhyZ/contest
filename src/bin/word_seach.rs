use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn exist(mut board: Vec<Vec<char>>, word: String) -> bool {
        // pre check
        let mut board_w_cnt = HashMap::new();
        for i in 0..board.len() {
            for j in 0..board[0].len() {
                *board_w_cnt.entry(board[i][j]).or_insert(0) += 1;
            }
        }

        let mut word_w_cnt = HashMap::new();
        for ch in word.chars() {
            *word_w_cnt.entry(ch).or_insert(0) += 1;

            if word_w_cnt[&ch] > *board_w_cnt.get(&ch).unwrap_or(&0) {
                return false;
            }
        }

        if word.len() == 0 || board.len() == 0 || board[0].len() == 0 {
            return false;
        }
        let m = board.len();
        let n = board[0].len();

        // 回溯法 + 深度优先搜索
        for i in 0..m {
            for j in 0..n {
                if Self::dfs(&mut board, &word, i, j, 0) {
                    return true;
                }
            }
        }
        return false;
    }

    pub fn dfs(
        board: &mut Vec<Vec<char>>,
        word: &String,
        i: usize,
        j: usize,
        index: usize,
    ) -> bool {
        if index == word.len() {
            return true;
        }
        let m = board.len();
        let n = board[0].len();
        // 如果当前位置的字符与预期不一致，则返回false
        if i >= m || j >= n || board[i][j] != word.chars().nth(index).unwrap() {
            return false;
        }
        // 将当前位置的字符存储
        let temp = board[i][j];
        board[i][j] = '#';
        // 四个方向开始查看是否与预期一致
        let directions = [[0, 1], [1, 0], [0, -1], [-1, 0]];
        for [dx, dy] in directions {
            if Self::dfs(
                board,
                word,
                (i as isize + dx) as usize,
                (j as isize + dy) as usize,
                index + 1,
            ) {
                return true;
            }
        }
        board[i][j] = temp;
        return false;
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    struct Case {
        board: Vec<Vec<char>>,
        word: String,
        expected: bool,
        name: String,
    }

    #[test]
    fn test_solution() {
        let test_cases = vec![
            Case {
                board: vec![
                    vec!['A', 'B', 'C', 'E'],
                    vec!['S', 'F', 'C', 'S'],
                    vec!['A', 'D', 'E', 'E'],
                ],
                word: "ABCCED".to_string(),
                expected: true,
                name: "case1".to_string(),
            },
            Case {
                board: vec![vec!['a']],
                word: "a".to_string(),
                expected: true,
                name: "case2".to_string(),
            },
            Case {
                board: vec![vec!['a']],
                word: "ab".to_string(),
                expected: false,
                name: "case3".to_string(),
            },
        ];

        for case in test_cases {
            let result = Solution::exist(case.board, case.word);
            assert_eq!(case.expected, result, "{}", case.name);
        }
    }
}

fn main() {}
