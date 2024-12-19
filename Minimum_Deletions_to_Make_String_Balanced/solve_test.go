package Minimum_Deletions_to_Make_String_Balanced

import "testing"

func Test_minimumDeletions(t *testing.T) {

	tests := []struct {
		s    string
		want int
	}{
		{
			s:    "aababbab",
			want: 2,
		},
		{
			s:    "bbaaaaabb",
			want: 2,
		},
		{
			s:    "bbbb",
			want: 0,
		},
		{
			s:    "aaaa",
			want: 0,
		},
	}
	for _, tt := range tests {
		tt := tt
		t.Run("min delete ", func(t *testing.T) {
			if got := minimumDeletions(tt.s); got != tt.want {
				t.Errorf("minimumDeletions() = %v, want %v", got, tt.want)
			}
		})
	}
}
