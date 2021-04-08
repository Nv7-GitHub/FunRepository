package gobench

func Pow(a, b int) int {
	out := 1
	for i := 0; i < b; i++ {
		out = Multiply(out, a)
	}
	return out
}
