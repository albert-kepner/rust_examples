fn main() {
    println!("{}", to_weird_case(&"Hello world of the unknown"));
}

fn to_weird_case(s: &str) -> String {
    let words: Vec<String> = s
        .split(' ') // Iterator<Item = &str>
        .map(|s| s.to_string()) // Convert to String
        .collect();

    let mut x_words: String = String::new();

    for (iw, word) in words.into_iter().enumerate() {
        // println!("{}", word);
        let mut char_vec: String = String::new();
        for (i, w) in word.chars().enumerate() {
            let part: String;
            if i % 2 == 0 {
                part = w.to_uppercase().collect();
            } else {
                part = w.to_lowercase().collect();
            }
            char_vec.push_str(&part);
        }
        if iw > 0 {
            x_words.push_str(&String::from(' '));
        }
        x_words.push_str(&char_vec);
    }

    return x_words;
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::to_weird_case;

    #[test]
    fn test_to_weird_case_one_word() {
        assert_eq!(to_weird_case("This"), "ThIs");
        assert_eq!(to_weird_case("is"), "Is");
        assert_eq!(to_weird_case("String"), "StRiNg");
    }
    #[test]
    fn test_to_weird_case_multiple_words() {
        assert_eq!(to_weird_case("This is a test"), "ThIs Is A TeSt");
        assert_eq!(to_weird_case("Weird string case"), "WeIrD StRiNg CaSe");
    }
}
