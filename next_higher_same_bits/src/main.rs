fn main() {
    println!("Hello, world!");
}

fn next_higher(n: i32) -> i32 {
    let target_ones = count_ones(n);
    let mut found_ones = 0;
    for i in n + 1.. {
        found_ones = count_ones(i);
        if found_ones == target_ones {
            return i;
        }
    }
    return 0;
}

fn count_ones(mut n: i32) -> i32 {
    let mut count = 0;
    while n > 0 {
        count += n & 1;
        n >>= 1;
    }
    return count;
}


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn basic_tests() {
        assert_eq!(next_higher(128), 256);
        assert_eq!(next_higher(1), 2);
        assert_eq!(next_higher(1022), 1279);
        assert_eq!(next_higher(127), 191);
        assert_eq!(next_higher(1253343), 1253359);
    }
}
