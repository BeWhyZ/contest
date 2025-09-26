79.[Word Search](https://leetcode.com/problems/word-search/)

Given an m x n grid of characters board and a string word, return true if word exists in the grid.

The word can be constructed from letters of sequentially adjacent cells, where adjacent cells are horizontally or vertically neighboring. The same letter cell may not be used more than once.


Input: board = [["A","B","C","E"],["S","F","C","S"],["A","D","E","E"]], word = "ABCCED"
Output: true

Input: board = [["A","B","C","E"],["S","F","C","S"],["A","D","E","E"]], word = "SEE"
Output: true

Input: board = [["A","B","C","E"],["S","F","C","S"],["A","D","E","E"]], word = "ABCB"
Output: false
 

Constraints:

m == board.length
n = board[i].length
1 <= m, n <= 6
1 <= word.length <= 15
board and word consists of only lowercase and uppercase English letters.
 

Follow up: Could you use search pruning to make your solution faster with a larger board?


---
暴力解决方案，合法的cell，当当前字母存在于word中，且相邻cell中有下一个字母，则该cell是合法的。
找到符合word 第一个字母的cell，依次来遍历可能性。
从第一个字母开始，下一个也是合法cell，且下一个合法cell未记录在当前路径上。
若能找到尾端则存在，否则不存在