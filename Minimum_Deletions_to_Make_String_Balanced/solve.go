package Minimum_Deletions_to_Make_String_Balanced

func minimumDeletions(s string) int {
	// dp
	// i < j and s[i]=='b' and s[j]=='a'
	// insure that there is no 'a' is in front of 'b'

	// define the dp, the dp[i] present the min deleteion of s[0:i]
	// so the dp[i] = dp[i-1] if s[i]!='a' else dp[i] = dp[i-1]+1
	// if dp[i-1] == 0,so dp[i] = dp[i-1] else,

	// init the base case
	// dp[0] = 0

	// recuirence
	// s[j:i]

	dp := make([]int, len(s))
	if len(s) < 2 {
		return 0
	}
	hasDiff := false
	for i, v := range s {
		if i == 0 {
			continue
		}
		if s[i-1] != s[i] {
			hasDiff = true
		}
		if v != 'a' {
			dp[i] = dp[i-1]
		} else {
			if hasDiff {
				dp[i] = dp[i-1] + 1
			} else {
				dp[i] = dp[i-1]
			}
		}
	}

	return dp[len(s)-1]
}
