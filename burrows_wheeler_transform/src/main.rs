fn main() {
    let mut items = vec!["x", "y", "a", "b", "c", "d", "e"];
    let marker: char = 0x1F as char;
    let mut marker_str: String= marker.to_lowercase().collect();
    items.push(&marker_str);

    // println!("unsorted: {:?}", items);

    items.sort();

    // println!("sorted: {:?}", items);

    let result = encode("Hello, world!");
    println!("results = {:?}", result);

    let decoded = decode(&result.0, result.1);

    println!("Decoded = {}", decoded);

}

use std::collections::HashMap;

pub fn encode(s: &str) -> (String, usize) {
    let marker: char = 0x1F as char;
    let mut char_vec: Vec<char> = Vec::new();
    
    for (i, c) in s.chars().enumerate() {
        char_vec.push(c);
    }
    // char_vec.push(marker);

    println!("encode results:");

    let mut slist: Vec<String> = Vec::new();
    let original: String = char_vec.iter().collect();
    for i in 0..char_vec.len() {

        slist.push(char_vec.iter().collect());
        char_vec.rotate_left(1);
        
    }

    for i in 0..char_vec.len() {
        println!("{:?}", slist[i]);
    }

    println!("");
    slist.sort();

    for i in 0..char_vec.len() {
        println!("{:?}", slist[i]);
        
    }
    let mut result_position: usize = 0;
    let position_opt: Option<usize> = slist.iter().position(|x| x == &original );
    if let Some(position) = position_opt {
        if position > 0 {
            result_position = position;
        }
        
        println!("\nPosition of original string = {}", position);
    }
    // Extract the last column from the sorted strings:
    let mut last_column: String = String::new();
    for i in 0..char_vec.len() {
        let last_char = slist[i].chars().last();
        if let Some(l_char) = last_char {
            if l_char != marker {
                last_column.push(l_char);
            }
        }
    }
    println!("last_column = {}", last_column);
    return (last_column, result_position);
}

pub fn decode(bwt_str: &str, result_position: usize) -> String {
    //# Step 1: Reconstruct the First column (F) by sorting the Last column (L)
    // L = list(bwt_str)
    // F = sorted(L)
    let mut last_col: Vec<char> = Vec::new();
    for c in bwt_str.chars() {
        last_col.push(c);
    }
    let mut first_col: Vec<char> = last_col.clone();
    first_col.sort();

    println!("last_col: {:?}", last_col);
    println!("first_col: {:?}", first_col);

    //# Step 2: Calculate occurrence ranks for both columns
    let last_col_ranks = compute_ranks(&last_col);
    let first_col_ranks = compute_ranks(&first_col);

    println!("last_col_ranks: {:?}", last_col_ranks);
    println!("first_col_ranks: {:?}", first_col_ranks);


    //# Create a fast lookup map: maps a (char, rank) tuple to its row index in column F
    let mut f_to_index: HashMap<(char, usize),usize> = HashMap::new();
    for (idx, item) in first_col_ranks.iter().enumerate() {
        f_to_index.insert(*item, idx);
    }

    //# Step 3: Begin tracing backwards starting from the terminal character in column F
    let mut decoded_reversed: Vec<char> = Vec::new();
    let mut current_row: usize = result_position;

    for i in 0..bwt_str.len() {
        //# Grab the character in column L that precedes our current position
        let prev_item = last_col_ranks[current_row];

        decoded_reversed.push(prev_item.0);

        //# Use LF-mapping: Jump to the row in F where this specific (char, rank) lives
        current_row = f_to_index[&prev_item]

    }

    //# Step 4: Reverse the gathered characters
    decoded_reversed.reverse();
    let result: String = decoded_reversed.iter().collect();


    return result;
}

// # Helper function to tag each character with its local occurrence count (rank)
// def compute_ranks(char_list):
//     counts = {}
//     ranked_list = []
//     for char in char_list:
//         rank = counts.get(char, 0)
//         ranked_list.append((char, rank))
//         counts[char] = rank + 1
//     return ranked_list

fn compute_ranks(char_list: &Vec<char>) -> Vec<(char,usize)>{
    let mut counts: HashMap<char,usize> = HashMap::new();
    let mut ranked_list: Vec<(char, usize)> = Vec::new();
    for c in char_list {
        let rank = counts.get(c).unwrap_or(&0);
        ranked_list.push ( (*c, *rank) );
        counts.insert(*c, *rank + 1);
    }

    return ranked_list;
}
















#[cfg(test)]
mod sample_tests {
    use super::{decode, encode};

    #[test]
    fn sample_tests_encode() {
        assert_eq!(encode("bananabar"), ("nnbbraaaa".to_owned(), 4));
        assert_eq!(encode("Humble Bundle"), ("e emnllbduuHB".to_owned(), 2));
        assert_eq!(encode("Mellow Yellow"), ("ww MYeelllloo".to_owned(), 1));
    }

    // #[test]
    // fn sample_tests_decode() {
    //     assert_eq!(decode("nnbbraaaa", 4), "bananabar");
    //     assert_eq!(decode("e emnllbduuHB", 2), "Humble Bundle");
    //     assert_eq!(decode("ww MYeelllloo", 1), "Mellow Yellow");
    // }
}
