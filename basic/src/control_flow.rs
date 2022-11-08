
// Loop循环 循环n轮的值
pub fn fib_loop(n: u8) -> i32 {
    let mut a = 1;
    let mut b = 1;
    let mut i = 0;

    println!("\nfib_loop start...");
    loop {
        next_fib(&mut a, &mut b);
        i += 1;
        println!("next fibonacci number: {}", b);
        // 条件分支
        if i >= n {
            // 返回结果 退出循环
            return b
        }
    }
}

// for循环
// 可以用于任何实现了 IntoIterator trait 的数据结构
pub fn fib_for(n: u8) {
    let (mut a, mut b) = (1, 1);

    println!("\nfib_for start...");

    // 2..n 是range的操作, 包含 2 <= x < n 的值
    // 不支持负数
    // 可以省略上标和下标
    // arr[..]
    // arr[0..n]
    for _i in 2..n {
        next_fib(&mut a,&mut b);
        println!("next fibonacci number: {}", b);
    }
}

// while循环
pub fn fib_while(n: u8) {
    let (mut a, mut b, mut i) = (1, 1, 0u8);
    
    println!("\nwfib_while start...");
    while i < n {
        next_fib(&mut a,&mut b);
        i += 1;
        println!("next fibonacci number: {}", b);
    }
}

pub fn next_fib(a: &mut i32, b: &mut i32) {
    let c = *a + *b;
    *a = *b;
    *b = c;
}

pub fn run() {
    let n = 10;
    println!("\n>>>control_flow start...");
    fib_loop(n);
    fib_for(n);
    fib_while(n);
}

#[test]
fn test() {
    // fib_loop(2);
    assert_eq!(2, fib_loop(1));
    assert_eq!(3, fib_loop(2));
    assert_eq!(5, fib_loop(3));
    assert_eq!(8, fib_loop(4));
    assert_eq!(13, fib_loop(5));
    assert_eq!(21, fib_loop(6));
    assert_eq!(34, fib_loop(7));
}