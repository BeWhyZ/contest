package longestpalindromicsubstring

import "testing"

func Test_longestPalindrome(t *testing.T) {
	tests := []struct {
		arg  string
		want string
	}{
		{
			arg:  "babad",
			want: "bab",
		},
		// {
		// 	arg:"cbbd",
		// 	want:"bb",
		// },
		// {
		// 	arg:"aaaa",
		// 	want:"aaaa",
		// },
	}
	for _, tt := range tests {
		tt := tt
		t.Run("longestpalindromicsubstring", func(t *testing.T) {
			if got := longestPalindrome(tt.arg); got != tt.want {
				t.Errorf("longestPalindrome() = %v, want %v", got, tt.want)
			}
		})
	}
}
