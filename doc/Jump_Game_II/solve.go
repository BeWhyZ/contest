package jump_game_ii

func jump(nums []int) int {
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

	// dp[i]表示跳数 每一次都跳最远
	// i表示跳到这个位置，dp[i]表示跳到当前位置所需要的最小步数
	// baseCase dp[0]=0
	// recurrence relation: dp[i] = min(dp[i-1]+nums[i-1], dp[i-2]+nums[i-2], ... , dp[i-k]+nums[i-k])
	if len(nums) == 0 {
		return 0
	}
	dp := make([]int, len(nums), len(nums))

	for i := 0; i < len(nums); i++ {
		// 计算dp[i]
		for k := 0; k < i; k++ {
			if k+nums[k] >= i && (dp[i] == 0 || dp[k]+1 < dp[i]) {
				dp[i] = dp[k] + 1
			}
		}
	}
	return dp[len(nums)-1]
}
