fn main() {
    println!("Hello, world!");
    let data = String::from("foobar099");
    let char_arr = data.chars().collect::<Vec<char>>();
    let mut char_arr2:Vec<char> = vec![];
    let char_len = char_arr.len();
    for i in (0..char_len).rev() {
        char_arr2.push(char_arr[i]);
    }
    let s: String = char_arr2.into_iter().collect();
    println!("s = {s}");
    println!("{}",increment("123"));
}

fn increment_string(s: &str) -> String {
    return match s.rfind(|c: char| !c.is_ascii_digit()) {
        Some(i) => format!("{}{}", &s[..=i], increment(&s[i + 1..])),
        None => format!("{}", increment(&s)),
    };
}

fn increment(s: &str) -> String {
    let char_arr = s.chars().collect::<Vec<char>>();
    let mut char_arr2:Vec<char> = vec![];
    let char_len = char_arr.len();
    let mut carry: bool = true;
    for i in (0..char_len).rev() {
        let mut c = char_arr[i];
        if carry {
            c = increment_char(&c);
            if c != '0' {
                carry = false;
            }
        }
        char_arr2.push(c)
    }
    if carry {
        char_arr2.push('1');
    }
    char_arr2.reverse();
    let s: String = char_arr2.iter().collect();
    return s;
}

fn increment_char(c: &char) -> char {
    match c {
        '0' => '1',
        '1' => '2',
        '2' => '3',
        '3' => '4',
        '4' => '5',
        '5' => '6',
        '6' => '7',
        '7' => '8',
        '8' => '9',
        '9' => '0',
        _ => '.',
    }
}

// fn increment_number(s1: &str) -> String {
//     let width = &s1.len();
//     let inc1: i32 =
//     match s1.parse::<i32>() {
//         Ok(n) => n+1,
//         Err(_e) => 1,
//     };
//     return format!("{:0>width$}",inc1);
// }

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::increment_string;

    fn dotest(s: &str, expected: &str) {
        let actual = increment_string(s);
        assert!(actual == expected, 
            "Test failed with s = \"{s}\"\nExpected \"{expected}\"\nBut got \"{actual}\"")
    }
    
    #[test]
    fn sample_tests() {
        dotest("foo", "foo1");
        dotest("foobar001", "foobar002");
        dotest("foobar1", "foobar2");
        dotest("foobar00", "foobar01");
        dotest("foobar99", "foobar100");
        dotest("foobar099", "foobar100");
        dotest("", "1");
    }
}
