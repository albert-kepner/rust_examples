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
    let _solution = slide_puzzle(&test3);
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

#[derive(Debug)]
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
    Finished,
}

#[derive(Debug)]
struct PuzzleState {
    goal: Goal,
    puzzle: Vec<Vec<u8>>,
    freeze_array: Vec<Vec<bool>>,
    goal_array: Vec<Vec<u8>>,
    zero_loc: Loc,
    completed_rows: usize,
    completed_cols: usize,
    square_size: usize,
    last_number_placed: u8,
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
        last_number_placed: 0,
    };
    let mut continue_solving = true;
    while continue_solving {
        continue_solving = update_goal(&mut puzzle_state);
    }
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

fn move_zero(puzzle_state: &mut PuzzleState, target_loc: Loc) {
    let zero_row = puzzle_state.zero_loc.row;
    let zero_col = puzzle_state.zero_loc.col;
    let target_row = target_loc.row;
    let target_col = target_loc.col;

    let is_adjacent = (zero_row == target_row && zero_col.abs_diff(target_col) == 1)
        || (zero_col == target_col && zero_row.abs_diff(target_row) == 1);

    if !is_adjacent {
        panic!(
            "Target location ({}, {}) is not adjacent to zero location ({}, {})",
            target_row, target_col, zero_row, zero_col
        );
    }

    let temp = puzzle_state.puzzle[target_row][target_col];
    puzzle_state.puzzle[target_row][target_col] = 0;
    puzzle_state.puzzle[zero_row][zero_col] = temp;

    puzzle_state.zero_loc.row = target_row;
    puzzle_state.zero_loc.col = target_col;
}

fn zero_placement(from_location: &Loc, to_location: &Loc) -> Vec<Loc> {
    let mut placements = Vec::new();

    let row_diff = to_location.row as isize - from_location.row as isize;
    let col_diff = to_location.col as isize - from_location.col as isize;

    // If we need to move vertically
    if row_diff < 0 {
        // Need to move up, so zero should be above (row - 1)
        placements.push(Loc {
            row: from_location.row - 1,
            col: from_location.col,
        });
    } else if row_diff > 0 {
        // Need to move down, so zero should be below (row + 1)
        placements.push(Loc {
            row: from_location.row + 1,
            col: from_location.col,
        });
    }

    // If we need to move horizontally
    if col_diff < 0 {
        // Need to move left, so zero should be to the left (col - 1)
        placements.push(Loc {
            row: from_location.row,
            col: from_location.col - 1,
        });
    } else if col_diff > 0 {
        // Need to move right, so zero should be to the right (col + 1)
        placements.push(Loc {
            row: from_location.row,
            col: from_location.col + 1,
        });
    }

    placements
}

fn update_goal(puzzle_state: &mut PuzzleState) -> bool {
    println!("Updating goal from {:?}", puzzle_state.goal);
    match puzzle_state.goal {
        Goal::Start => {
            puzzle_state.goal = Goal::RowNext;
            true
        }
        Goal::RowNext => {
            puzzle_state.goal = Goal::Finished;
            true
        }
        Goal::Finished => {
            println!("Puzzle solved!");
            false
        }
        _ => false
    }
}