package jump_game_ii

func jumpGreedy(nums []int) int {
	// 使用DP来解
	// 嵌套n层括号, 每一层括号数量均是ns
	// 第几层，同时有一个特点对于任意个，dp[i] = dp[i-1] +

	// - Define the Subproblem:
	// Let dp[i] represent all combinations of well-formed parentheses with i pairs.

	// - Base Case:
	// dp[0] is an empty string: [""].

	// - Recurrence Relation:
	// For each i from 1 to n, we can form dp[i] by iterating through all possible splits of i pairs into two parts: j pairs and i-j-1 pairs.
	// For each combination of j pairs and i-j-1 pairs, we can form a new combination by wrapping the j pairs in a pair of parentheses and concatenating it with the i-j-1 pairs.

	// - Implementation:
	// Use a list to store the results for each dp[i].
	// Iterate through all possible splits and combine the results from the subproblems.

	// 每一步走最远，并求出可走最远区间内最大的nums[i]， 如果nums[i]+range > len(nums) break
	if len(nums) < 2 {
		return 0
	}
	// current_end 当前走到的最远位置
	// farthest 能走到的最远位置
	// jumps 跳数
	jumps, current_end, farthest := 0, 0, 0
	for i := 0; i < len(nums)-1; i++ {
		farthest = max(farthest, i+nums[i])
		if i == current_end {
			jumps++
			current_end = farthest
			if current_end >= len(nums)-1 {
				break
			}
		}
	}

	return jumps
}

func max(a, b int) int {
	if a > b {
		return a
	}
	return b
}
