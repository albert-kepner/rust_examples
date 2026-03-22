fn main() {
    println!("Hello, world!");
}

use std::collections::VecDeque;

fn path_finder(maze: &str) -> Option<u32> {
    println!("maze: \n{}",maze);
    let mut m = matrix(maze);
    print_matrix(&m);
    let n: usize = m.len();
    if n == 1 {
        return Some(0);
    }
    let mut queue: VecDeque<(usize, usize, u32)> = VecDeque::new();
    queue.push_back( (0, 0, 0) );
    m[0][0] = -1;
    while queue.len() > 0 {
        let node: Option<(usize, usize, u32)> = queue.pop_front();
        match node {
            Some( (i, j, depth) ) => {
                let steps: Vec<(usize, usize, u32)> = next_steps(i, j, depth, &mut m);
                for (ix, jx, depthx) in steps {
                    println!("Step to ({ix}, {jx}) at depth: {depthx}");
                    queue.push_back( (ix, jx, depthx ));
                    if ix == n - 1 && jx == n - 1 {
                        return Some(depthx);
                    }
                }
            },
            None => {
                return None;
            },
        }
    }
    None
}

fn next_steps(i: usize, j: usize, depth: u32, m: &mut Vec<Vec<i32>>) -> Vec<(usize, usize, u32)> {
    let mut steps: Vec<(usize, usize, u32)> = vec![];
    let n = m.len();
    if i > 0 && m[i-1][j] == 0 {
        steps.push( (i-1, j, depth+1) );
        m[i-1][j] = depth as i32 + 1;
    }
    if j > 0 && m[i][j-1] == 0 {
        steps.push( (i, j-1, depth+1) );
        m[i][j-1] = depth as i32 + 1;
    }
    if i < n - 1 && m[i+1][j] == 0 {
        steps.push( (i+1, j, depth+1) );
        m[i+1][j] = depth as i32 + 1;
    }
    if j < n - 1 && m[i][j+1] == 0 {
        steps.push( (i, j+1, depth+1) );
        m[i][j+1] = depth as i32 + 1;
    }

    return steps;
}

fn print_matrix(matrix: &Vec<Vec<i32>>) -> () {
    for row in matrix {
        println!("{:?}", row);
    }
}

fn matrix(maze: &str) -> Vec<Vec<i32>> {
    let lines: Vec<&str> = maze.lines().collect();
    let mut matrix: Vec<Vec<i32>> = vec![];
    for i in 0..lines.len() {
        let mut row: Vec<i32> = vec![];
        let line = lines[i];
        for c in line.chars()  {
            let x: i32;
            if c == 'W' {
                x = -1;
            } else {
                x = 0;
            }
            row.push(x);
        }
        matrix.push(row);
    }
    return matrix;
}


// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::path_finder;


    #[test]
    fn fixed_tests() {
        assert_eq!(path_finder("."), Some(0), "\nYour answer (left) is not the expected answer (right).");
        assert_eq!(path_finder(".W.\n.W.\n..."), Some(4), "\nYour answer (left) is not the expected answer (right).");
        assert_eq!(path_finder(".W.\n.W.\nW.."), None, "\nYour answer (left) is not the expected answer (right).");
        assert_eq!(path_finder("......\n......\n......\n......\n......\n......"), Some(10), "\nYour answer (left) is not the expected answer (right).");
        assert_eq!(path_finder("......\n......\n......\n......\n.....W\n....W."), None, "\nYour answer (left) is not the expected answer (right).");
    }
}
