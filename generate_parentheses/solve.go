package generate_parentheses

func generateParenthesis(n int) []string {
	// 使用DP来解
	// 嵌套n层括号, 每一层括号数量均是n
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

	dp := []string{}
	if n == 0 {
		return dp
	}
	for i := 1; i < n; i++ {
		for j := 1; j < n; j++ {

		}
	}

	return []string{}
}
