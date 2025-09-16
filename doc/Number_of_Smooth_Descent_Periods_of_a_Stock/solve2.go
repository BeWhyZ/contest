package Number_of_Smooth_Descent_Periods_of_a_Stock

func getDescentPeriods2(prices []int) int64 {
	if len(prices) == 0 {
		return 0
	}
	var perid int64 = 0
	// 使用DP来解
	// 依次遍历，length表示连续符合rule的最长长度， 计算length之内的所有可能性，若无则跳过
	// 如果num[i]+1 == num[i-1],perid+=2 else perid++
	i, lth := 1, 0

	for i < len(prices) {
		if prices[i-1]-1 == prices[i] {
			lth++
			i++
		} else if lth > 0 {
			// 计算lth之内的可能性
			lth++
			perid += int64(lth*(lth+1)/2) - 1
			i += lth
			lth = 0
		} else {
			i++
		}

	}

	return perid
}