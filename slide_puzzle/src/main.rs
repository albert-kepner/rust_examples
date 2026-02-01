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

fn zero_placement_path(
    puzzle_state: &PuzzleState,
    from_location: &Loc,
    target_locations: &[Loc],
) -> Vec<Loc> {
    use std::collections::{VecDeque, HashMap};

    let rows = puzzle_state.puzzle.len();
    let cols = puzzle_state.puzzle[0].len();
    let start = &puzzle_state.zero_loc;

    // BFS to find shortest path
    let mut queue = VecDeque::new();
    let mut visited = vec![vec![false; cols]; rows];
    let mut parent: HashMap<(usize, usize), (usize, usize)> = HashMap::new();

    queue.push_back((start.row, start.col));
    visited[start.row][start.col] = true;

    // Helper to check if location is valid and not blocked
    let is_valid = |r: usize, c: usize| -> bool {
        if r >= rows || c >= cols {
            return false;
        }
        // Avoid from_location
        if r == from_location.row && c == from_location.col {
            return false;
        }
        // Avoid frozen locations
        if puzzle_state.freeze_array[r][c] {
            return false;
        }
        true
    };

    // Check if we've reached any target location
    let is_target = |r: usize, c: usize| -> bool {
        target_locations.iter().any(|loc| loc.row == r && loc.col == c)
    };

    let mut found_target: Option<(usize, usize)> = None;

    while let Some((row, col)) = queue.pop_front() {
        if is_target(row, col) {
            found_target = Some((row, col));
            break;
        }

        // Try all four adjacent directions
        let neighbors = [
            (row.wrapping_sub(1), col), // up
            (row + 1, col),              // down
            (row, col.wrapping_sub(1)), // left
            (row, col + 1),              // right
        ];

        for (next_row, next_col) in neighbors {
            if is_valid(next_row, next_col) && !visited[next_row][next_col] {
                visited[next_row][next_col] = true;
                parent.insert((next_row, next_col), (row, col));
                queue.push_back((next_row, next_col));
            }
        }
    }

    if let Some((end_row, end_col)) = found_target {
        // Reconstruct path
        let mut path = Vec::new();
        let mut current = (end_row, end_col);

        while current != (start.row, start.col) {
            path.push(Loc {
                row: current.0,
                col: current.1,
            });
            current = *parent.get(&current).unwrap();
        }

        path.reverse();
        path
    } else {
        panic!(
            "No valid path found from zero location ({}, {}) to any target location while avoiding from_location ({}, {})",
            start.row, start.col, from_location.row, from_location.col
        );
    }
}

fn move_number(puzzle_state: &mut PuzzleState, from_location: Loc, to_location: Loc) {
    let mut current_location = from_location;
    println!("Moving number from ({}, {}) to ({}, {})", current_location.row, current_location.col, to_location.row, to_location.col);
    let mut step = 0;   
    // Keep moving the number until it reaches the target location
    while current_location.row != to_location.row || current_location.col != to_location.col {
        // Get the ideal placement locations for zero
        let target_placements = zero_placement(&current_location, &to_location);
        // for tp in &target_placements {
        //     println!("Target placement ({}, {}) ", tp.row, tp.col);
        // }

        // Find the shortest path from zero to one of the target placements
        // avoiding the current number location and frozen cells
        let path = zero_placement_path(puzzle_state, &current_location, &target_placements);
        // for loc in &path {
        //     println!("Path step to ({}, {})", loc.row, loc.col);
        // }

        // Move zero along the path
        for loc in path {
            move_zero(puzzle_state, loc);
            // println!("After moving zero:");
            // print_puzzle(&puzzle_state.puzzle);
        }
        step += 1;
        if step > 200 {
            panic!("Too many steps moving number, possible infinite loop");
        }
        let zero_loc_row = puzzle_state.zero_loc.row;
        let zero_loc_col = puzzle_state.zero_loc.col;

        // Now swap zero with the number (this moves the number one step)
        move_zero(puzzle_state, Loc {
            row: current_location.row,
            col: current_location.col,
        });

        // Update current location - the number is now where zero was before the swap
        // Zero's current location is now where the number was
        current_location = Loc {
            row: zero_loc_row,
            col: zero_loc_col,
        };
    }
    puzzle_state.freeze_array[to_location.row][to_location.col] = true;
    println!("Number moved to ({}, {}) and frozen.", to_location.row, to_location.col);
}

fn update_goal(puzzle_state: &mut PuzzleState) -> bool {
    println!("Updating goal from {:?}", puzzle_state.goal);
    match puzzle_state.goal {
        Goal::Start => {
            puzzle_state.goal = Goal::RowNext;
            true
        }
        Goal::RowNext => {
            let from_location = find_item(&puzzle_state.puzzle, puzzle_state.last_number_placed + 1);
            let column = puzzle_state.last_number_placed as usize % puzzle_state.square_size;
            let to_location = Loc { row: puzzle_state.completed_rows, col: column };
            move_number(puzzle_state,
                from_location,
                to_location);
            puzzle_state.last_number_placed += 1;
            if puzzle_state.last_number_placed as usize % puzzle_state.square_size >= puzzle_state.square_size - 2 {
                puzzle_state.goal = Goal::Finished;
            }
            true
        }
        Goal::Finished => {
            println!("Puzzle solved!");
            print_puzzle(&puzzle_state.puzzle);
            false
        }
        _ => false
    }
}