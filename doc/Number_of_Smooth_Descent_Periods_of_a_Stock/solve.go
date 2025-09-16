package Number_of_Smooth_Descent_Periods_of_a_Stock

func getDescentPeriods(prices []int) int64 {
	if len(prices) == 0 {
		return 0
	}
	var perid int64 = 0
	// 使用DP来解

	dp := make([]int64, len(prices), len(prices))

	// dp[i] 已i开始的数字下降序列个数
	// dp[i] = dp[i-1]-1 if dp[i-1]>1  else 开始推算, all = sum(dp[i])
	for i := 0; i < len(prices); i++ {
		dp[i] = 1
		if i != 0 && dp[i-1] > 1 {
			dp[i] = dp[i-1] - 1
		} else {
			//
			for j := i + 1; j < len(prices); j++ {
				if prices[j-1] == prices[j]+1 {
					dp[i]++
				} else {
					break
				}
			}
		}

		perid += dp[i]
	}

	return perid
}
