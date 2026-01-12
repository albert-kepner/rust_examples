fn main() {
    println!("Hello, nonogram!");
    get_data(); 
}

type Clues = Vec<Vec<u8>>;

pub fn encode(sol: &[&[u8]]) -> (Clues, Clues) {
    let row_clues = get_row_clues(sol);
    let col_clues = get_col_clues(sol);
    (col_clues, row_clues)
}

fn get_row_clues(sol: &[&[u8]]) -> Clues {
    let mut row_clues: Clues = Vec::new();

    for row in sol {
        let mut clues: Vec<u8> = Vec::new();
        let mut count = 0;

        for &cell in *row {
            if cell == 1 {
                count += 1;
            } else if count > 0 {
                clues.push(count);
                count = 0;
            }
        }

        if count > 0 {
            clues.push(count);
        }

        row_clues.push(clues);
    }

    row_clues
}

fn get_col_clues(sol: &[&[u8]]) -> Clues {
    let mut col_clues: Clues = Vec::new();
    let num_cols = sol[0].len();

    for col_idx in 0..num_cols {
        let mut clues: Vec<u8> = Vec::new();
        let mut count = 0;

        for row in sol {
            let cell = row[col_idx];
            if cell == 1 {
                count += 1;
            } else if count > 0 {
                clues.push(count);
                count = 0;
            }
        }

        if count > 0 {
            clues.push(count);
        }

        col_clues.push(clues);
    }

    col_clues
}

fn print_ng(sol: &[&[u8]]) {
    for row in sol {
        for &cell in *row {
            if cell == 1 {
                print!("█");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

fn get_data() {
    let my_nonogram: &[&[u8]] = &[
            &[0, 0, 0, 1, 0, 0, 0, 1, 1, 0],
            &[0, 0, 1, 1, 1, 0, 1, 1, 1, 1],
            &[0, 0, 1, 1, 1, 1, 1, 1, 1, 1],
            &[0, 0, 0, 1, 1, 1, 1, 1, 1, 0],
            &[0, 0, 0, 0, 0, 1, 1, 0, 0, 0],
            &[0, 1, 0, 0, 0, 0, 1, 1, 0, 0],
            &[1, 0, 1, 0, 0, 0, 1, 1, 0, 0],
            &[1, 1, 1, 0, 0, 1, 1, 0, 0, 0],
            &[1, 1, 1, 0, 0, 1, 1, 1, 0, 1],
            &[1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
        ];
    let ng = my_nonogram;
    print_ng(ng);
    let (col_clues, row_clues) = encode(ng);
    println!("Column Clues:");
    print_clues(&col_clues);
    println!("Row Clues:");
    print_clues(&row_clues);
}



fn print_clues(clues: &Clues) {
    for clue in clues {
        for &num in clue {
            print!("{} ", num);
        }
        println!();
    }
}   