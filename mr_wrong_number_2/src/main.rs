#![allow(dead_code)]

fn main() {
    println!("Hello, world!");
}

use regex::Regex;

pub fn find_out_mr_wrong<'a>(conversation: &[&'a str]) -> Option<&'a str> {
    todo!("Your code here!")
}

struct State<'a> {
    persons: Vec<Person<'a>>
}

enum Statement {
    AbsPosition { position: usize },
    RelPosition { relative: i32, person_index: usize },
}

struct Person<'a> {
    name: &'a str,
    index: usize,
    statements: Vec<Statement>,
}

fn parse_conversation<'a>(conversation: &[&'a str]) -> State<'a> {
    let mut persons = Vec::new();
    let re1 = Regex::new(r"^(\w+):I'm in (\d+)(\w+) position.$").unwrap();

    for (index,line) in conversation.iter().enumerate() {
        println!("({}) {}", index, line);
        if let Some(caps) = re1.captures(line) {
            let name = caps.get(1).unwrap().as_str();
            let postion: uzize = caps.get(2)
                .unwrap()
                .parse()
                .expect("position should be a valid integer");
            println!("Person: {} states position at {}", name, position);)
        } else {
            println!("Skipped!")
        }
    }

    State { persons }
}


// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

mod sample_tests {
    use super::find_out_mr_wrong;

    #[test]
    fn basic_tests() {
        for (conversation, expected) in SAMPLE_TEST_CASES {
            let actual = find_out_mr_wrong(conversation);
            // assert_eq!(actual, expected);
        }
    }

    const SAMPLE_TEST_CASES: [(&[&str], Option<&str>); 10] = [
        (
            &[
                "John:I'm in 1st position.",
                "Peter:I'm in 2nd position.",
                "Tom:I'm in 1st position.",
                "Peter:The man behind me is Tom.",
            ],
            Some("Tom"),
        ),
        (
            &[
                "John:I'm in 1st position.",
                "Peter:I'm in 2nd position.",
                "Tom:I'm in 1st position.",
                "Peter:The man in front of me is Tom.",
            ],
            Some("John"),
        ),
        (
            &[
                "John:I'm in 1st position.",
                "Peter:There is 1 people in front of me.",
                "Tom:There are 2 people behind me.",
                "Peter:The man behind me is Tom.",
            ],
            Some("Tom"),
        ),
        (
            &[
                "John:The man behind me is Peter.",
                "Peter:There is 1 people in front of me.",
                "Tom:There are 2 people behind me.",
                "Peter:The man behind me is Tom.",
            ],
            None,
        ),
        (
            &[
                "Dowfls:There is 0 people behind me.",
                "Dowfls:I'm in 4th position.",
                "Ljiyxbmr:I'm in 2nd position.",
                "Ljiyxbmr:There is 1 people in front of me.",
                "Cvvugb:There are 2 people in front of me.",
                "Cvvugb:There is 1 people behind me.",
                "Tzjlvruhk:The man behind me is Dowfls.",
                "Tzjlvruhk:There are 2 people in front of me.",
            ],
            None,
        ),
        (
            &[
                "Tom:The man behind me is Bob.",
                "Bob:The man in front of me is Tom.",
                "Bob:The man behind me is Gary.",
                "Gary:The man in front of me is Bob.",
                "Fred:I'm in 1st position.",
            ],
            Some("Fred"),
        ),
        (&["Wrong:The man behind me is Wrong."], Some("Wrong")),
        (
            &[
                "Charles:The man behind me is Gavin.",
                "Gavin:I'm in 1st position.",
                "Ken:The man in front of me is Gavin.",
                "Charles:The man in front of me is Gavin.",
            ],
            Some("Charles"),
        ),
        (
            &[
                "Greg:I'm in 1st position.",
                "Daniel:There are 2 people in front of me.",
                "Ramone:I'm in 3rd position.",
                "Daniel:There are 2 people behind me.",
            ],
            Some("Daniel"),
        ),
        (
            &[
                "Frodo:I'm in 3rd position.",
                "Gollum:I'm in 3rd position.",
                "Sam:The man behind me is Frodo.",
                "Gollum:The man behind me is Frodo.",
            ],
            Some("Gollum"),
        ),
    ];
}

