fn main() {
    println!("Hello, world!");
}

use std::collections::HashSet;
use std::collections::VecDeque;

type Coord = (usize, usize);

fn connected_values(arr: &[Vec<u8>], val: u8, coord: Coord) -> Vec<Coord> {
    let mut result: Vec<Coord> = Vec::new();
    if arr[coord.0][coord.1] != val {
        println!("No match to val, returning early...");
        return result;
    }
    let mut open: VecDeque<Coord> = VecDeque::new();
    let mut goal_set: HashSet<Coord> = HashSet::new();
    open.push_back(coord);

    while !open.is_empty() {
        if let Some(next) = open.pop_front() {
            if arr[next.0][next.1] == val {
                if !goal_set.contains(&next) {
                    goal_set.insert(next);
                    let connected: Vec<Coord> = find_connections(arr, next);
                    for ix in 0..connected.len() {
                        open.push_back(connected[ix]);
                    }
                }
            }
        }
    }
    result = goal_set.into_iter().collect();
    result
}

fn find_connections(arr: &[Vec<u8>], coord: Coord) -> Vec<Coord> {
    let mut result: Vec<Coord> = Vec::new();
    let row_len = arr.len();
    let col_len = arr[0].len();
    let irow: i32 = coord.0 as i32;
    let jcol: i32 = coord.1 as i32;
    for i in -1..2 {
        let row = irow + i;
        if row >= 0 && row < row_len as i32 {
            for j in -1..2 {
                let col: i32 = jcol + j;
                if col >= 0 && col < col_len as i32 {
                    let next: Coord = (row as usize, col as usize);
                    result.push(next);
                }
            }
        }
    }
    return result;
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
#[allow(unused_variables)]
mod tests {
    use super::connected_values;
    use itertools::Itertools;
    // type Coord = (usize, usize);
    use super::Coord;

    fn dotest(arr: &[Vec<u8>], val: u8, coord: Coord, expected: &[Coord]) {
        try_equal(
            &connected_values(arr, val, coord)
                .iter()
                .sorted()
                .map(|x| *x)
                .collect::<Vec<Coord>>(),
            expected,
            val,
            coord,
        )
    }

    fn try_equal(connected: &Vec<Coord>, expected: &[Coord], val: u8, coord: Coord) -> () {
        if connected != expected {
            let err_msg = "NOT EQUAL";
            println!("connected: {:?}", connected);
            println!("expected: {:?}", expected);
            println!("{err_msg} with val = {}, coord = {:?}", val, coord);
        }
    }

    #[test]
    fn fixed_tests() {
        let example_arr = &[
            vec![1, 0, 2, 0, 2, 1],
            vec![1, 0, 2, 1, 5, 7],
            vec![4, 1, 1, 0, 1, 9],
        ];
        dotest(
            example_arr,
            1,
            (0, 0),
            &[(0, 0), (1, 0), (1, 3), (2, 1), (2, 2), (2, 4)],
        );
        dotest(example_arr, 2, (0, 2), &[(0, 2), (1, 2)]);
        dotest(example_arr, 0, (0, 0), &[]);

        let arr1 = &[
            vec![0, 0, 0, 1, 3, 4, 0, 3],
            vec![0, 2, 0, 0, 2, 0, 0, 5],
            vec![0, 0, 0, 2, 0, 1, 1, 1],
            vec![2, 3, 4, 1, 3, 1, 0, 0],
            vec![0, 1, 5, 1, 6, 0, 2, 0],
            vec![2, 0, 2, 3, 1, 1, 1, 1],
        ];
        dotest(arr1, 0, (4, 2), &[]);
        dotest(
            arr1,
            0,
            (0, 0),
            &[
                (0, 0),
                (0, 1),
                (0, 2),
                (0, 6),
                (1, 0),
                (1, 2),
                (1, 3),
                (1, 5),
                (1, 6),
                (2, 0),
                (2, 1),
                (2, 2),
                (2, 4),
            ],
        );
    }
}
