package jump_game_ii

import "testing"

func Test_jump(t *testing.T) {

	tests := []struct {
		nums []int
		want int
	}{
		// {
		// 	nums: []int{2, 3, 1, 1, 4},
		// 	want: 2,
		// },
		// {
		// 	nums: []int{2, 3, 1, 1, 4},
		// 	want: 2,
		// },
		// {
		// 	nums: []int{},
		// 	want: 0,
		// },
		// {
		// 	nums: []int{1},
		// 	want: 0,
		// },
		{
			nums: []int{1, 2, 1, 1, 1},
			want: 3,
		},
		{
			nums: []int{1, 1, 1, 1, 1},
			want: 4,
		},
	}
	for _, tt := range tests {
		tt := tt
		t.Run("jump game ii ", func(t *testing.T) {
			if got := jump(tt.nums); got != tt.want {
				t.Errorf("jump() = %v, want %v", got, tt.want)
			}
		})

		t.Run("jump game ii with greedy", func(t *testing.T) {
			if got := jumpGreedy(tt.nums); got != tt.want {
				t.Errorf("jumpGreedy() = %v, want %v", got, tt.want)
			}
		})
	}
}
