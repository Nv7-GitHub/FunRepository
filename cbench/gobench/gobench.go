package gobench

func Add(a, b int) int {
	return a + b
}

func Multiply(a, b int) int {
	out := 0
	for i := 0; i < b; i++ {
		out = Add(out, a)
	}
	return out
}
