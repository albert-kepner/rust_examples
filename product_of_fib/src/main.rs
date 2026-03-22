fn main() {
    println!("Hello, world!");
}


fn product_fib(prod: u64) -> (u64, u64, bool) {
    let mut product: u64 = 0;
    let mut fib1: u64 =0;
    let mut fib2: u64 = 1;
    while product < prod {
        (fib1, fib2, product) = next_product_of(fib1, fib2);
        if product >= prod {
            let equal_flag: bool = product == prod;
            return (fib1, fib2, equal_flag);
        }
    }
    return (0, 1, false);
}

fn next_product_of(fib_n: u64, fib_n_plus: u64) -> (u64, u64, u64) {
    let fib_next = fib_n + fib_n_plus;
    return (fib_n_plus, fib_next, fib_n_plus * fib_next);
}

fn dotest(prod: u64, exp: (u64, u64, bool)) -> () {
    assert_eq!(product_fib(prod), exp)
}

#[test]
fn basics_product_fib() {
    dotest(4895, (55, 89, true));
    dotest(5895, (89, 144, false));
}
