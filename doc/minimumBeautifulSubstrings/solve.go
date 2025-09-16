package minimumBeautifulSubstrings

import (
	"math"
	"strconv"
)

func isBeautiful(s string) bool {
	if s[0] == '0' {
		return false
	}
	num, err := strconv.ParseInt(s, 2, 64)
	if err != nil || num < 1 {
		return false
	}
	for num%5 == 0 {
		num /= 5
	}

	return num == 1
}

func minimumBeautifulSubstrings(s string) int {
	// 使用DP来解
	// def
	// baseCase
	// recurrence relation
	// dp[i] = dp[i-1]
	// d

	// failed
	// dp[i]表示 最大字串长度为i的最小美丽字串长度
	// dp[0] = 0
	// dp[1] = 1
	// dp[i] = dp[i-1] 在不超过i的情况下与相邻进行合并，并且合并后的仍然是稳定的

	// success
	// dp[i]表示s[0:i]的最小美丽字符串的长度
	// dp[i] = dp[i-1] + 1， 1通过下述条件来得到，if s[j:i]是美丽字符串，那么dp[i] = min(dp[i],dp[j] + 1)

	// baseCase dp[0] = 0, dp[i] = unreachableState=当前是求最小，因此该值是最大

	if s == "" || s == "1" {
		return -1
	}
	n := len(s)
	dp := make([]int, n+1, n+1)
	for i := range dp {
		dp[i] = math.MaxInt32
	}
	dp[0] = 0
	for i := 1; i < n+1; i++ {
		for j := 0; j < i; j++ {
			if isBeautiful(s[j:i]) && dp[j]+1 < dp[i] {
				dp[i] = dp[j] + 1
			}
		}
	}
	if dp[n] == math.MaxInt32 {
		return -1 // If no valid partition exists
	}

	return dp[n]
}

func min(a, b int) int {
	if a < b {
		return a
	}
	return b
}
