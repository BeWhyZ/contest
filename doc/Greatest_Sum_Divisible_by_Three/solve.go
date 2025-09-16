package Greatest_Sum_Divisible_by_Three

func maxSumDivThree(nums []int) int {
	nm := 0

	// dp[i] 表示nums[0:i] 中最大的数 that is divisible by 3
	// dp[i] = dp[i-1]+ nums[i] if dp[i-1]+nums[i] % 3 == 0 else dp[i-1]

	return nm
}
