use num_bigint::BigInt;


fn fib(n1: i32) -> BigInt {
    // Iterative O(n) with BigInt
    if n1 == 0 {
        return BigInt::from(0i32);
    }
    let mut a = BigInt::from(0i32);
    let mut b = BigInt::from(1i32);
    let n = n1.abs();
    for _ in 0..n {
        let next = &a + &b;
        a = b;
        b = next;
    }
    if n1 < 0 && n1 % 2 == 0 {
        return BigInt::from(0i32) - &a
    }
    a
}

fn main() {
    let f1000 = fib(1000);
    println!("F(1000) has {} digits", f1000.to_string().len());
}

