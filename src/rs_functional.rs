fn main() {
    println!("Hello from Rust");

    fn add(x: i32, y: i32) -> i32 {
        x + y
    }

    fn multiply(x: i32, y: i32) -> i32 {
        x * y
    }

    // 1
    let a = add(1, 1);
    println!("{}", a);

    // 2
    let add2 = |x, y| x + y;
    let b = add2(2, 2);
    println!("{}", b);

    // 3
    let c = |x, y| x + y;
    let c_result = c(3, 3);
    println!("{}", c_result);

    // 4
    println!("{}", (|a, b| a + b)(4, 4));

    // 5 send function
    fn apply_func<F>(f: F, x: i32, y: i32) -> i32
    where
        F: Fn(i32, i32) -> i32,
    {
        f(x, y)
    }

    let d = apply_func(add, 5, 5);
    println!("{}", d);

    let e = apply_func(multiply, 6, 6);
    println!("{}", e);

    let f = apply_func(|x, y| x * y + 1, 7, 7);
    println!("{}", f);

    // 6 return function
    fn return_func(x: i32) -> impl Fn(i32, i32) -> i32 {
        if x + x >= 15 {
            |x, y| x + y
        } else {
            |x, y| x * y
        }
    }

    let my_fun = return_func(8);
    let g = my_fun(8, 8);
    println!("{}", g);
}
