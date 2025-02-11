package main

// Author: Oliver Lau <ola@ct.de>

import (
	"fmt"
	"math/big"
)

func fib(n int) {
	a := big.NewInt(0)
	b := big.NewInt(1)
	for i := 0; i < n; i++ {
		fmt.Print(a, " ")
		a.Add(a, b)
		a, b = b, a
	}
}

func main() {
	fib(100)
}
