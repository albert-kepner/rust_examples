#![allow(dead_code)]

fn main() {
    println!("Hello, world!");
}

use regex::Regex;
use std::collections::HashSet;

pub fn find_out_mr_wrong<'a>(conversation: &[&'a str]) -> Option<&'a str> {
    let blessed_names: Vec<&'a str> = bless_these_names(conversation);
    let mut state: State<'a> = parse_conversation(conversation);
    if let Some(name) = state.solve() {
        println!("Mr. Wrong identified by solve(): {}", name);
        return Some(get_name(&blessed_names, name));
    }
    // else {
    // if let Some(name) = state.solve2() {
    //     println!("Mr. Wrong identified by solve2(): {}", name);
    //     return Some(get_name(&blessed_names, name));
    // } else {
    //     println!("No contradictions found, unable to identify Mr. Wrong.");
    //     return None;
    // }
    // }
    // exclude_supporting_pairs
    let maybe_liars = state.exclude_supporting_pairs();
    None
}

fn get_name<'a>(blessed_names: &Vec<&'a str>, name: &'a str) -> &'a str {
    if let Some(index) = blessed_names.iter().position(|&n| n == name) {
        return blessed_names[index];
    } else {
        return "Unknown";
    }
}

fn bless_these_names<'a>(conversation: &[&'a str]) -> Vec<&'a str> {
    let mut blessed_names: Vec<&'a str> = Vec::new();
    let re = Regex::new(r"^(\w+):").unwrap();
    for line in conversation {
        if let Some(caps) = re.captures(line) {
            let name = caps.get(1).unwrap().as_str();
            if !blessed_names.contains(&name) {
                blessed_names.push(name);
            }
        }
    }
    blessed_names
}

struct State<'a> {
    person_names: Vec<&'a str>,
    persons: Vec<Person<'a>>,
    trials: Vec<Trial>,
}

impl<'a> State<'a> {
    fn new() -> Self {
        State {
            person_names: Vec::new(),
            persons: Vec::new(),
            trials: Vec::new(),
        }
    }
    fn make_trials(&mut self) {
        for person_index in 0..self.persons.len() {
            let person_name = self.persons[person_index].name;
            self.trials
                .push(Trial::new(person_index, self.persons.len(), person_name));
        }
    }
    fn person_index(&mut self, name: &'a str) -> usize {
        if let Some(index) = self.person_names.iter().position(|&n| n == name) {
            return index;
        } else {
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
        self.make_trials();

        None
    }
    // fn solve2(&self) -> Option<&'a str> {
    //     // Implement an alternative logic to solve the puzzle based on the collected statements.
    //     let mut possible_liar_indexes1: Vec<usize> = Vec::new();
    //     let mut possible_liar_indexes2: Vec<usize> = Vec::new();
    //     let mut possible_liar_indexes3: Vec<usize> = Vec::new();
    //     let mut test_the_liar: bool = false;
    //     let verbose: bool = false;
    //     for i in 0..2 {
    //         if i == 1 {
    //             test_the_liar = true;
    //         }
    //         for trial in &self.trials {
    //             let (other_lies, liar_lies, consistent) =
    //                 trial.is_contradictory(&self, &test_the_liar);
    //             if verbose {
    //                 println!(
    //                     "After is_contradictory({}): !other_lies {} consistent: {}",
    //                     trial.liar_index, !other_lies, consistent
    //                 );
    //             }

    //             if consistent && !test_the_liar {
    //                 if verbose {
    //                     println!(
    //                         "FOUND CONSISTENT CONFIGURATION ******************************************************************************************"
    //                     );
    //                 }
    //                 possible_liar_indexes3.push(trial.liar_index);
    //             }
    //             if !test_the_liar && !other_lies {
    //                 if verbose {
    //                     println!("Found lack of contradiction for others...");
    //                 }
    //                 possible_liar_indexes1.push(trial.liar_index);
    //             }
    //             if test_the_liar && liar_lies {
    //                 if verbose {
    //                     println!("Found contradiction by assumed liar.");
    //                 }
    //                 possible_liar_indexes2.push(trial.liar_index);
    //             }
    //         }
    //     }
    //     // If statements are only consistent for one liar, we have the villian!
    //     if possible_liar_indexes3.len() == 1 {
    //         let index = possible_liar_indexes3[0];
    //         println!("unique reason 3");
    //         return Some(self.person_names[index]);
    //     } else if possible_liar_indexes1.len() == 1 {
    //         let index = possible_liar_indexes1[0];
    //         println!("unique reason 1");
    //         return Some(self.person_names[index]);
    //     } else if possible_liar_indexes2.len() == 1 {
    //         let index = possible_liar_indexes2[0];
    //         println!("unique reason 2");
    //         return Some(self.person_names[index]);
    //     } else {
    //         if let Some(person_index) = self.exclude_supporting_pairs() {
    //             println!("unique reason 4");
    //             return Some(self.person_names[person_index]);
    //         }
    //     }
    //     None
    // }

    fn exclude_supporting_pairs(&self) -> Vec<usize> {
        let mut after_pairs: Vec<(usize, usize)> = Vec::new();
        let mut before_pairs: Vec<(usize, usize)> = Vec::new();
        let mut person_indexes: HashSet<usize> = (0..self.persons.len()).collect();
        let verbose: bool = false;
        for person in &self.persons {
            for statement in &person.statements {
                match statement {
                    Statement::RelPosition {
                        relative,
                        person_index,
                    } => {
                        if *relative == -1 {
                            before_pairs.push((person.index, *person_index));
                        } else if *relative == 1 {
                            // put after_pairs in reverse order so they can be matched directly with before pairs.
                            after_pairs.push((*person_index, person.index))
                        }
                    }
                    _ => {
                        // noop, ignore
                    }
                }
            }
        }
        if verbose {
            println!("before_pairs:");
            for pair in &before_pairs {
                println!("{:?}", pair);
            }
            println!("after_pairs:");
            for pair in &after_pairs {
                println!("{:?}", pair);
            }
            println!("person_indexes before:");
            for index in &person_indexes {
                println!("{:?}", index);
            }
        }
        for pair in before_pairs {
            // When before_pair matches after_pair we have
            // two persons who mutually support each other's positions.
            // Since only the liar lies, each two such persons cannot be the liar.
            if after_pairs.contains(&pair) {
                person_indexes.remove(&pair.0);
                person_indexes.remove(&pair.1);
            }
        }
        if verbose {
            println!("person_indexes after excluding supporting pairs:");
            for index in &person_indexes {
                println!("{:?}", index);
            }
        }
        // If exactly one person is not covered by supporting pairs,
        // that person must be the liar.
        println!(
            "exclude_supporting_pairs: persons: {} possible_liars: {}",
            self.persons.len(),
            person_indexes.len()
        );
        return person_indexes.into_iter().collect();
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
                Statement::RelPosition {
                    relative,
                    person_index,
                } => {
                    // Check if this person tries to claim their position relative to themselves.
                    if *person_index == self.index {
                        return true; // Contradiction found.
                    }

                    // Check if this relative position contradicts any previous statements.
                    for prev_statement in &relatives {
                        match prev_statement {
                            Statement::RelPosition {
                                relative: prev_relative,
                                person_index: prev_person_index,
                            } => {
                                if *person_index == *prev_person_index
                                    && *relative != *prev_relative
                                {
                                    return true; // Contradiction found.
                                }
                            }
                            _ => {}
                        }
                    }
                }
                _ => {
                    // otherwise ignore for now.
                }
            }
            relatives.push(statement);
        }
        false
    }
}

struct Trial {
    liar_index: usize,
    num_people: usize,
}

impl Trial {
    fn new(liar_index: usize, num_people: usize, person_name: &str) -> Self {
        let verbose: bool = false;
        if verbose {
            println!(
                "Creating new trial with liar_index: {} liar: {} and num_people: {}",
                liar_index, person_name, num_people
            );
        }
        Trial {
            liar_index,
            num_people,
        }
    }

    fn make_assignments(&self) -> Vec<Assignment> {
        let mut assignments = Vec::new();
        for person_index in 0..self.num_people {
            assignments.push(Assignment::new(person_index, self.num_people));
        }
        assignments
    }

    fn claim_position(
        &self,
        assignments: &mut Vec<Assignment>,
        person_index: usize,
        is_liar: bool,
        position: usize,
    ) -> (bool, bool) {
        let mut changed: bool = false;
        let mut has_contradiction: bool = false;
        let assignment = &mut assignments[person_index];
        // If this person is not the liar, then their statement is true, so we can set their position to the claimed index.
        if !is_liar {
            if !assignment.possible_positions.contains(&position) {
                // If this claimed position is not possible for the Trial we have a contradiction
                has_contradiction = true;
            } else if assignment.position.is_none() {
                assignment.position = Some(position);
                assignment.possible_positions = vec![position];
                changed = true;
            }
        } else { // have liar, claimed position must be false
            if let Some(current_position) = assignment.position {
                if current_position == position {
                    // liar claiming the actual position is a contradiciton
                    has_contradiction = true;
                }
            } else {
                // If this person is the liar, then their statement is false, so we can remove the claimed position from their possible positions.
                if assignment.possible_positions.contains(&position) {
                    assignment.possible_positions.retain(|&x| x != position);
                    changed = true;
                }
            }
        }
        return (changed, has_contradiction);
    }

    fn is_contradictory(&self, state: &State) -> bool {
        // Implement logic to consider statements in the trial and determine if
        // the assumption of a specific liar leads to a contradiction based on the statements.
        let mut assignments: Vec<Assignment> = self.make_assignments();
        let mut has_contradiction = false;
        let verbose: bool = false;

        loop {
            let mut changed = false;
            for person in &state.persons {
                let person_index = person.index;
                let is_liar: bool = person_index == self.liar_index;
                if verbose {
                    println!(
                        "Considering person_index {}: {} ( liar_index = {} is_liar = {})",
                        person_index, person.name, self.liar_index, is_liar
                    );
                }
                for statement in &person.statements {
                    match statement {
                        Statement::AbsPosition { position } => {
                            if verbose {
                                println!(
                                    "Person {} claims absolute position: {}",
                                    person.name, position
                                );
                            }
                            (changed, has_contradiction) = self.claim_position(
                                &mut assignments,
                                person_index,
                                is_liar,
                                *position,
                            );
                        }
                        Statement::ReversePosition { from_end } => {
                            // let assignment = &mut assignments[person_index];
                            let position: usize = state.persons.len() - *from_end;
                            if verbose {
                                println!(
                                    "Person {} claims reverse position: {}",
                                    person.name, position
                                );
                            }
                            (changed, has_contradiction) = self.claim_position(
                                &mut assignments,
                                person_index,
                                is_liar,
                                position,
                            );
                        }
                        Statement::RelPosition {
                            relative,
                            person_index,
                        } => {
                            if verbose {
                                println!(
                                    "Person {} claims relative position: {} relative to person_index {}",
                                    person.name, relative, person_index
                                );
                            }
                            let other_person_index = *person_index;
                            let this_person_index = person.index;
                            let (contradiction, change_flag) = self.infer_relative(
                                &mut assignments,
                                this_person_index,
                                other_person_index,
                                *relative,
                            );
                            if change_flag {
                                changed = true;
                            }
                            if contradiction {
                                has_contradiction = true;
                            }
                        }
                    }
                    if verbose {
                        for assignment in &assignments {
                            println!(
                                "Assignment for person_index {}: possible_positions: {:?} position = {:?}",
                                assignment.person_index,
                                assignment.possible_positions,
                                assignment.position
                            );
                        }
                    }
                }
            }
            // Consider all the exact position assignments we have so far, and for each person assigned, remove that position from the possible positions of all other people.
            let exact_assignments: Vec<(usize, usize)> = assemble_assignments(&assignments);
            let (new_change, new_contradiction) =
                propagate_assignments(&exact_assignments, &mut assignments);
            changed = changed || new_change;
            if new_contradiction {
                has_contradiction = true;
            }

            if !changed || has_contradiction {
                break; // No changes made, stop the loop.
            }
        } // End of loop to consider statements,
        if verbose {
            println!(
                "END LOOP ****** Trial  with liar_index: {}  has_contradiction: {}",
                self.liar_index, has_contradiction
            );
            for assignment in &assignments {
                println!(
                    "Assignment for person_index {}: possible_positions: {:?} position = {:?}",
                    assignment.person_index, assignment.possible_positions, assignment.position
                );
            }
        }

        return has_contradiction;
    }

    fn infer_relative(
        &self,
        assignments: &mut Vec<Assignment>,
        this_person_index: usize,
        other_person_index: usize,
        relative: i32,
    ) -> (bool, bool) {
        // Implement logic to infer new constraints based on relative position statements.
        // Return true if any changes were made to the assignments, false otherwise.
        let mut changed = false;
        let mut contradiction: bool = false;

        if this_person_index == other_person_index {
            contradiction = true;
            return (contradiction, changed);
        }

        let (this_assignment, other_assignment) = if this_person_index < other_person_index {
            let (left, right) = assignments.split_at_mut(other_person_index);
            (&mut left[this_person_index], &mut right[0])
        } else {
            let (left, right) = assignments.split_at_mut(this_person_index);
            (&mut right[0], &mut left[other_person_index])
        };
        // if this positon is known, we can constrain the other person's position based on the relative statement.
        if let Some(this_position) = this_assignment.position {
            let other_position = match relative {
                -1 => this_position.checked_sub(1),
                1 => this_position.checked_add(1),
                _ => None,
            };
            if let Some(other_position) = other_position {
                if !other_assignment
                    .possible_positions
                    .contains(&other_position)
                {
                    contradiction = true;
                    return (contradiction, changed);
                }
                if other_assignment.position.is_none() {
                    other_assignment.position = Some(other_position);
                    other_assignment.possible_positions = vec![other_position];
                    changed = true;
                }
            } else {
                // position out of range (if zero)
                println!("Contradiction: relative postition: zero 0003");
                contradiction = true;
                return (contradiction, changed);
            }
        }
        // if the other person's position is known, we can constrain this person's position based on the relative statement.
        if let Some(other_position) = other_assignment.position {
            let this_position = match relative {
                -1 => other_position.checked_add(1),
                1 => other_position.checked_sub(1),
                _ => None,
            };
            if let Some(this_position) = this_position {
                if !this_assignment.possible_positions.contains(&this_position) {
                    contradiction = true;
                    return (contradiction, changed);
                }
                if this_assignment.position.is_none() {
                    this_assignment.position = Some(this_position);
                    this_assignment.possible_positions = vec![this_position];
                    changed = true;
                }
            } else {
                // position out of range (if zero)
                println!("Contradiction: relative postition: zero 0004");
                contradiction = true;
                return (contradiction, changed);
            }
        }

        // If this person is not the liar, then their statement is true, so we can
        // constrain their position relative to the other person's position.

        // If relative is -1, then the other person is in front of this person.
        // max other_person_position = max this_person_position - 1
        // Further:
        // min other person_position = min this_person_position - 1

        // If relative is 1, then the other person is behind this person.
        // max other_person_position = max this_person_position + 1
        // Further:
        // min other person_position = min this_person_position + 1
        if !changed {
            // let max_this: Option<&usize> = this_assignment.possible_positions.iter().max();
            // let max_other: Option<&usize> = other_assignment.possible_positions.iter().max();
            // let min_this: Option<&usize> = this_assignment.possible_positions.iter().min();
            // let min_other: Option<&usize> = other_assignment.possible_positions.iter().min();
            let (max_this, max_other, min_this, min_other) =
                &self.get_ranges(&this_assignment, &other_assignment);
            // max_allowed (other)
            let max_allowed: Option<usize> = if relative == 1 {
                if let Some(_) = max_this {
                    max_this.unwrap().checked_add(1)
                } else {
                    None
                }
            } else {
                if let Some(_) = max_this {
                    max_this.unwrap().checked_sub(1)
                } else {
                    None
                }
            };
            let max_allowed_this: Option<usize> = if relative == 1 {
                if let Some(_) = max_this {
                    max_other.unwrap().checked_sub(1)
                } else {
                    None
                }
            } else {
                if let Some(_) = max_this {
                    max_other.unwrap().checked_add(1)
                } else {
                    None
                }
            };
            // min_allowed (other)
            let min_allowed: Option<usize> = if relative == 1 {
                if let Some(_) = min_this {
                    min_this.unwrap().checked_add(1)
                } else {
                    None
                }
            } else {
                if let Some(_) = min_this {
                    min_this.unwrap().checked_sub(1)
                } else {
                    None
                }
            };
            let min_allowed_this: Option<usize> = if relative == 1 {
                if let Some(_) = min_this {
                    min_other.unwrap().checked_sub(1)
                } else {
                    None
                }
            } else {
                if let Some(_) = min_this {
                    min_other.unwrap().checked_add(1)
                } else {
                    None
                }
            };
            match (max_allowed, max_other) {
                (Some(allowed), Some(other)) => {
                    if *other > allowed {
                        changed = true;
                        other_assignment
                            .possible_positions
                            .retain(|&x| x <= allowed);
                    }
                }
                _ => {
                    println!("Contradiction relative max 001");
                    contradiction = true;
                    return (contradiction, changed);
                }
            }
            match (max_allowed_this, max_this) {
                (Some(allowed), Some(other)) => {
                    if *other > allowed {
                        changed = true;
                        this_assignment.possible_positions.retain(|&x| x <= allowed);
                    }
                }
                _ => {
                    println!("Contradiction relative max 901");
                    contradiction = true;
                    return (contradiction, changed);
                }
            }
            match (min_allowed, min_other) {
                (Some(allowed), Some(other)) => {
                    if *other < allowed {
                        changed = true;
                        other_assignment
                            .possible_positions
                            .retain(|&x| x >= allowed);
                    }
                }
                _ => {
                    println!("Contradiction relative min 001");
                    contradiction = true;
                    return (contradiction, changed);
                }
            }
            match (min_allowed_this, min_this) {
                (Some(allowed), Some(other)) => {
                    if *other < allowed {
                        changed = true;
                        this_assignment.possible_positions.retain(|&x| x >= allowed);
                    }
                }
                _ => {
                    println!("Contradiction relative min 901");
                    contradiction = true;
                    return (contradiction, changed);
                }
            }
            if other_assignment.possible_positions.is_empty() {
                println!("Contradiction relative empty 002");
                contradiction = true;
                return (contradiction, changed);
            }
            if this_assignment.possible_positions.is_empty() {
                println!("Contradiction relative empty 902");
                contradiction = true;
                return (contradiction, changed);
            }
        }

        return (contradiction, changed);
    }

    fn get_ranges(
        &self,
        this_assignment: &Assignment,
        other_assignment: &Assignment,
    ) -> (Option<usize>, Option<usize>, Option<usize>, Option<usize>) {
        let max_this: Option<&usize> = this_assignment.possible_positions.iter().max();
        let max_other: Option<&usize> = other_assignment.possible_positions.iter().max();
        let min_this: Option<&usize> = this_assignment.possible_positions.iter().min();
        let min_other: Option<&usize> = other_assignment.possible_positions.iter().min();
        let ranges = (
            max_this.copied(),
            max_other.copied(),
            min_this.copied(),
            min_other.copied(),
        );
        // println!("---> ranges = {:?}", ranges);
        ranges
    }
}

// let exact_assignments: Vec<(usize, usize)> = assemble_assignments(&assignments);
fn assemble_assignments(assignments: &Vec<Assignment>) -> Vec<(usize, usize)> {
    let mut exact_assignments: Vec<(usize, usize)> = Vec::new();
    for (index, assignment) in assignments.iter().enumerate() {
        if let Some(position) = assignment.position {
            exact_assignments.push((index, position));
        }
    }
    exact_assignments
}

// let (new_change: bool, new_contradiction: bool) = propagate_assignments(&exact_assignments, &mut assignments);
fn propagate_assignments(
    exact_assignments: &Vec<(usize, usize)>,
    assignments: &mut Vec<Assignment>,
) -> (bool, bool) {
    let mut change: bool = false;
    let mut contradiction: bool = false;
    for (index, position) in exact_assignments {
        for index2 in 0..assignments.len() {
            if *index != index2 {
                let len_before: usize = assignments[index2].possible_positions.len();
                assignments[index2]
                    .possible_positions
                    .retain(|&x| x != *position);
                let len_after: usize = assignments[index2].possible_positions.len();
                if len_after != len_before {
                    change = true;
                }
                if len_after == 1 {
                    assignments[index2].position = Some(assignments[index2].possible_positions[0]);
                }
                if len_after == 0 {
                    contradiction = true;
                }
            }
        }
    }
    return (change, contradiction);
}

struct Assignment {
    position: Option<usize>,
    possible_positions: Vec<usize>,
    person_index: usize,
    num_people: usize,
}

impl Assignment {
    fn new(person_index: usize, num_people: usize) -> Self {
        Assignment {
            position: None,                                 // 1-based position of person if known.
            possible_positions: (1..=num_people).collect(), // collection of 1-based positions...
            person_index,
            num_people,
        }
    }
}

fn is_consistent(assignments: &Vec<Assignment>, num_people: usize) -> bool {
    let mut set: HashSet<usize> = HashSet::new();
    let mut undecided = 0;
    let verbose: bool = false;
    for assignment in assignments {
        match assignment.position {
            Some(position) => {
                set.insert(position);
            }
            None => {
                undecided += 1;
            }
        }
    }
    // If all but one position is assigned, the last one can be inferred...
    let consistent =
        set.len() == num_people || (((set.len() + 1) == num_people) && (undecided == 1));
    if verbose {
        println!(
            "is_consistent: set.len(): {} num_people {} consistent {} undecided {} set {:?}",
            set.len(),
            num_people,
            consistent,
            undecided,
            set
        );
    }
    return consistent;
}

fn parse_conversation<'a>(conversation: &[&'a str]) -> State<'a> {
    let mut state = State::new();
    let re1 = Regex::new(r"^(\w+):I'm in (\d+)(\w+) position.$").unwrap();
    let re2: Regex = Regex::new(r"^(\w+):There (?:is|are) (\d+) people? in front of me.$").unwrap();
    let re3: Regex = Regex::new(r"^(\w+):There (?:is|are) (\d+) people? behind me.$").unwrap();
    let re4: Regex = Regex::new(r"^(\w+):The man (in front of|behind) me is (\w+).$").unwrap();

    // println!("\n\nParsing conversation:");
    for (_index, line) in conversation.iter().enumerate() {
        // println!("({}) {}", index, line);
        if let Some(caps) = re1.captures(line) {
            let name = caps.get(1).unwrap().as_str();
            let position_str: &str = caps.get(2).unwrap().as_str();
            let position: usize = position_str.parse().unwrap();

            // println!("Person: {} states position at {}", name, position);
            let person_index = state.person_index(name);
            state.add_statement(person_index, Statement::AbsPosition { position });
        } else if let Some(caps) = re2.captures(line) {
            let name = caps.get(1).unwrap().as_str();
            let count_str: &str = caps.get(2).unwrap().as_str();
            let count: usize = count_str.parse().unwrap();

            // println!("Person: {} states there are {} people in front of them", name, count);
            let person_index = state.person_index(name);
            state.add_statement(
                person_index,
                Statement::AbsPosition {
                    position: count + 1,
                },
            );
        } else if let Some(caps) = re3.captures(line) {
            let name = caps.get(1).unwrap().as_str();
            let count_str: &str = caps.get(2).unwrap().as_str();
            let count: usize = count_str.parse().unwrap();

            // println!("Person: {} states there are {} people behind them", name, count);
            let person_index = state.person_index(name);
            state.add_statement(person_index, Statement::ReversePosition { from_end: count });
        } else if let Some(caps) = re4.captures(line) {
            let name = caps.get(1).unwrap().as_str();
            let direction = caps.get(2).unwrap().as_str();
            let other_person = caps.get(3).unwrap().as_str();

            // println!("Person: {} states the man {} me is {}", name, direction, other_person);
            let offset: i32 = if direction == "in front of" { -1 } else { 1 };
            let person_index = state.person_index(name);
            let other_person_index = state.person_index(other_person);
            state.add_statement(
                person_index,
                Statement::RelPosition {
                    relative: offset,
                    person_index: other_person_index,
                },
            );
        } else {
            eprintln!("Skipped line: {}", line);
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
        for (conversation, _expected) in SAMPLE_TEST_CASES_NEW {
            count += 1;
            let _actual = find_out_mr_wrong(conversation);
            warn_not_equal(count, _actual, _expected);
        }
    }

    fn warn_not_equal<T: std::fmt::Debug + PartialEq>(count: i32, actual: T, expected: T) {
        if actual != expected {
            eprintln!(
                "{} ******************* Warning: actual value {:?} does not match expected value {:?}",
                count, actual, expected
            );
        } else {
            println!(
                "{} ==================== Test passed: actual value {:?} matches expected value {:?}",
                count, actual, expected
            );
        }
    }

    const SAMPLE_TEST_CASES: [(&[&str], Option<&str>); 11] = [
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
        (
            &[
                "Hauejr:The man behind me is Apeiyb.",
                "Apeiyb:The man in front of me is Fbuye.",
                "Fbuye:The man behind me is Hauejr.",
                "Apeiyb:The man behind me is Hauejr.",
            ],
            Some("Apeiyb"),
        ),
    ];

    const SAMPLE_TEST_CASES_NEW: [(&[&str], Option<&str>); 2] = [
        (
            &[
                "Jgatt:The man in front of me is Wafxmw.",
                "Wafxmw:The man behind me is Jgatt.",
                "Pamqaabuj:There are 2 people in front of me.",
                "Dsfy:I'm in 4th position.",
            ],
            Some("Pamqaabuj"),
        ),
        (
            &[
                "Eteyjm:The man behind me is Ucuaei.",
                "Ucuaei:The man in front of me is Eteyjm.",
                "Vaqzcyicr:There is 1 people in front of me.",
                "Aujyuhoee:There are 3 people behind me.",
            ],
            Some("Vaqzcyicr"),
        ),
    ];
}
