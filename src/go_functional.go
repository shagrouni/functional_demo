package main

import "fmt"

func add(a int, b int) int {
	return a + b
}

func multiply(a int, b int) int {
	return a * b
}

func ApplyFunc(f func(int, int) int, x int, y int) int {
	return f(x, y)
}

func returnFunc(x int) func(int, int) int {
	if (x + x) >= 15 {
		return func(x int, y int) int { return x * y }
	} else {
		return func(x int, y int) int { return x + y }
	}
}

func main() {

	fmt.Println("Hello from Go")
	//1
	a := add(1, 1)
	fmt.Println(a)

	//2
	add := func(a int, b int) int { return a + b }
	b := add(2, 2)
	fmt.Println(b)

	//3
	c := (func(a int, b int) int { return a + b })(3, 3)
	fmt.Println(c)

	//4
	fmt.Println((func(a int, b int) int { return a + b })(4, 4))

	//5 send function

	d := ApplyFunc(add, 5, 5)
	fmt.Println(d)

	e := ApplyFunc(multiply, 6, 6)
	fmt.Println(e)

	f := ApplyFunc(func(a int, b int) int { return a*b + 1 }, 7, 7)

	fmt.Println(f)

	//6 return function

	myFun := returnFunc(8)
	g := myFun(8, 8)
	fmt.Println(g)
}
