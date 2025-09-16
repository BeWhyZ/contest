package longestpalindromicsubstring

func longestPalindrome(s string) string {
	// 使用DP来解
	// 从第n个字符串开始时，回串
	// 对上述求最大值即可求解
	// 状态转移公式,若以第k开始

	lS := len(s)
	if lS < 2 {
		return s
	}
	maxS := s[0:1]

	// DP
	// 当回文的中间字符为n时，有两种情况一种是左右两侧相等，第二种与一侧相等即可，即符合回文，并增长
	// 求最大dp值
	for i := range s {
		// 检查两侧相等情况
		l, r := i, i
		for l >= 0 && r < lS && s[l] == s[r] {
			if (r - l + 1) > len(maxS) {
				maxS = s[l : r+1]
			}
			l--
			r++
		}

		// 仅一侧相同
		l, r = i, i+1
		for l >= 0 && r < lS && s[l] == s[r] {
			if (r - l + 1) > len(maxS) {
				maxS = s[l : r+1]
			}
			l--
			r++
		}

	}
	return maxS
}
