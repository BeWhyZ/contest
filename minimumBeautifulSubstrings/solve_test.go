package minimumBeautifulSubstrings

import "testing"

func Test_minimumBeautifulSubstrings(t *testing.T) {
	type args struct {
		s string
	}
	tests := []struct {
		s    string
		want int
	}{
		// {
		// 	s:    "1011",
		// 	want: 2,
		// },
		{
			s:    "111",
			want: 3,
		},
		{
			s:    "0",
			want: -1,
		},
	}
	for _, tt := range tests {
		tt := tt
		t.Run("minimumBeautifulSubstrings", func(t *testing.T) {
			if got := minimumBeautifulSubstrings(tt.s); got != tt.want {
				t.Errorf("minimumBeautifulSubstrings() = %v, want %v, and s:%s", got, tt.want, tt.s)
			}
		})
	}
}
