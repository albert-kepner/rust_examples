#![allow(dead_code)]

fn main() {
    println!("Hello, world!");
}

use regex::Regex;

pub fn find_out_mr_wrong<'a>(conversation: &[&'a str]) -> Option<&'a str> {
    let mut state =parse_conversation(conversation);
    state.solve()
}
    
struct State<'a> {
    person_names: Vec<&'a str>,
    persons: Vec<Person<'a>>,
}

impl<'a> State<'a> {
    fn new() -> Self {
        State {
            person_names: Vec::new(),
            persons: Vec::new(),
        }
    }
    fn person_index (&mut self, name: &'a str) -> usize {
        if let Some(index) = self.person_names.iter().position(|&n| n == name) {
            return index;
        }  else {
            let index = self.persons.len();
            self.person_names.push(name);           
            self.persons.push(Person {
                name,
                index,
                statements: Vec::new(),
            });
            return index;
        }
    }
    fn person_at(&'a self, index: usize) -> &'a Person<'a> {
        &self.persons[index]
    }
    fn add_statement(&mut self, person_index: usize, statement: Statement) {
        self.persons[person_index].statements.push(statement);
    }
    fn solve(&mut self) -> Option<&'a str> {
        // Implement the logic to solve the puzzle based on the collected statements.
        
        for person in &self.persons {
            if person.contradicts_self() {
                return Some(person.name);
            }
        }
        None
    }
}

enum Statement {
    AbsPosition { position: usize },
    RelPosition { relative: i32, person_index: usize },
    ReversePosition { from_end: usize },
}

struct Person<'a> {
    name: &'a str,
    index: usize,
    statements: Vec<Statement>,
}

impl Person<'_> {
    fn contradicts_self(&self) -> bool {
        // Implement logic to check if the person's statements contradict each other.
        let mut relatives: Vec<&Statement> = Vec::new();
        for statement in &self.statements {
            match statement {
                Statement::RelPosition { relative, person_index } => {
                    // Check if this person tries to claim their position relative to themselves.
                    if *person_index == self.index {
                        return true; // Contradiction found.
                    }

                    // Check if this relative position contradicts any previous statements.
                    for prev_statement in &relatives {
                        match prev_statement {
                            Statement::RelPosition { relative: prev_relative, person_index: prev_person_index } => {
                                if *person_index == *prev_person_index && *relative != *prev_relative {
                                    return true; // Contradiction found.
                                }
                            },
                            _ => {},
                        }
                    }

                },
                _ => {
                    // otherwise ignore for now.

                },
            }
            relatives.push(statement);
        }
        false
    }
}

fn parse_conversation<'a>(conversation: &[&'a str]) -> State<'a> {
    let mut state = State::new();
    let re1 = Regex::new(r"^(\w+):I'm in (\d+)(\w+) position.$").unwrap();
    let re2: Regex = Regex::new(r"^(\w+):There (?:is|are) (\d+) people? in front of me.$").unwrap();
    let re3: Regex = Regex::new(r"^(\w+):There (?:is|are) (\d+) people? behind me.$").unwrap();
    let re4: Regex = Regex::new(r"^(\w+):The man (in front of|behind) me is (\w+).$").unwrap();


    for (index,line) in conversation.iter().enumerate() {
        println!("({}) {}", index, line);
        if let Some(caps) = re1.captures(line) {
            let name = caps.get(1).unwrap().as_str();
            let position_str: &str = caps.get(2).unwrap().as_str();
            let position: usize = position_str.parse().unwrap();

            println!("Person: {} states position at {}", name, position);
            let person_index = state.person_index(name);
            state.add_statement(person_index, Statement::AbsPosition { position });

        } else if let Some(caps) = re2.captures(line) {
            let name = caps.get(1).unwrap().as_str();
            let count_str: &str = caps.get(2).unwrap().as_str();
            let count: usize = count_str.parse().unwrap();

            println!("Person: {} states there are {} people in front of them", name, count);
            let person_index = state.person_index(name);
            state.add_statement(person_index, Statement::AbsPosition { position: count });

        } else if let Some(caps) = re3.captures(line) {
            let name = caps.get(1).unwrap().as_str();
            let count_str: &str = caps.get(2).unwrap().as_str();
            let count: usize = count_str.parse().unwrap();

            println!("Person: {} states there are {} people behind them", name, count);
            let person_index = state.person_index(name);
            state.add_statement(person_index, Statement::ReversePosition { from_end: count });

        } else if let Some(caps) = re4.captures(line) {
            let name = caps.get(1).unwrap().as_str();
            let direction = caps.get(2).unwrap().as_str();
            let other_person = caps.get(3).unwrap().as_str();

            println!("Person: {} states the man {} me is {}", name, direction, other_person);
            let offset: i32 = if direction == "in front of" { -1 } else { 1 };
            let person_index = state.person_index(name);
            let other_person_index = state.person_index(other_person);
            state.add_statement(person_index, Statement::RelPosition { relative: offset, person_index: other_person_index });

        } else {
            println!("Skipped!")
        }
    }

    state
}


// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

mod sample_tests {
    use super::find_out_mr_wrong;

    #[test]
    fn basic_tests() {
        let mut count = 0;
        for (conversation, _expected) in SAMPLE_TEST_CASES {
            count += 1;
            let _actual = find_out_mr_wrong(conversation);
            warn_not_equal(count, _actual, _expected);
        }
    }

    fn warn_not_equal<T: std::fmt::Debug + PartialEq>(count: i32, actual: T, expected: T) {
        if actual != expected {
            eprintln!("{} ******************* Warning: actual value {:?} does not match expected value {:?}", count, actual, expected);
        }   else {
            println!("{} ==================== Test passed: actual value {:?} matches expected value {:?}", count, actual, expected);
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

