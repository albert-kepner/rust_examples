fn main() {
    println!("slide puzzle");

    const TESTS: [&[&[u8]]; 3] = [
        &[&[4, 1, 3], &[2, 8, 0], &[7, 6, 5]],
        &[
            &[10, 3, 6, 4],
            &[1, 5, 8, 0],
            &[2, 13, 7, 15],
            &[14, 9, 12, 11],
        ],
        &[
            &[3, 7, 14, 15, 10],
            &[1, 0, 5, 9, 4],
            &[16, 2, 11, 12, 8],
            &[17, 6, 13, 18, 20],
            &[21, 22, 23, 19, 24],
        ],
    ];

    let test3 = puzzle_to_vec(TESTS[2]);
    print_puzzle(&test3);
    let loc = find_item(&test3, 0);
    println!("Location of 0: {:?}", loc);
}


fn puzzle_to_vec(arr: &[&[u8]]) -> Vec<Vec<u8>> {
    let mut result = Vec::new();
    for row in arr {
        let mut row_vec = Vec::new();
        for val in row.iter() {
            row_vec.push(*val);
        }
        result.push(row_vec);
    }
    result
}

fn print_puzzle(arr: &Vec<Vec<u8>>) {
    for row in arr {
        for &val in row {
            print!("[ {:>2} ]", val);
        }
        println!();
    }
    println!();
}

#[derive(Debug)]
struct Loc {
    row: usize,
    col: usize,
}

enum Goal {
    Start,
    RowNext,
    RowLastButOneMove,
    RowLastPlace,
    RowLastButOnePlace,
    EndRowRotate,
    ColumnLowMove,
    ColumnHighPlace,
    ColumnLowPlace,
    EndColumnRotate,
    LastCornerRotate,
}

struct PuzzleState {
    goal: Goal,
    puzzle: Vec<Vec<u8>>,
    freeze_array: Vec<Vec<bool>>,
    goal_array: Vec<Vec<u8>>,
    zero_loc: Loc,
    completed_rows: usize,
    completed_cols: usize,
    square_size: usize,
    number_moved_history: Vec<u8>,
}

/// Solves the slide puzzle represented by a 2D array `arr`.
pub fn slide_puzzle(arr: &[Vec<u8>]) -> Option<Vec<u8>> {
    let mut puzzle_state: PuzzleState = PuzzleState {
        goal: Goal::Start,
        puzzle: arr.to_vec(),
        freeze_array: vec![vec![false; arr[0].len()]; arr.len()],
        goal_array: vec![vec![0; arr[0].len()]; arr.len()],
        zero_loc: find_item(arr, 0),
        completed_rows: 0,
        completed_cols: 0,
        square_size: arr.len(),
        number_moved_history: Vec::new(),
    };
    return None;
}

fn find_item(arr: &[Vec<u8>], item: u8) -> Loc {
    for (i, row) in arr.iter().enumerate() {
        for (j, &val) in row.iter().enumerate() {
            if val == item {
                return Loc { row: i, col: j };
            }
        }
    }
    panic!("Item not found: {}", item);
}
