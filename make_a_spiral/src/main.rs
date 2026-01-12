fn main() {
    // println!("Hello, world!");
    for size in 3..12 {
        println!("Spiral of size {}:", size);
        let grid = spiralize(size);
        print_grid(&grid);
        println!();
    }
    // let grid = spiralize(7);
    // print_grid(&grid);
}

fn spiralize(size: usize) -> Vec<Vec<i8>> {
    let spiral = Spiral::new(size);
    spiral.make()
}

fn print_grid(grid: &Vec<Vec<i8>>) {
    for row in grid {
        for val in row {
            print!("{} ", val);
        }
        println!();
    }
}

#[derive(Debug)]
enum Direction {
    Right,
    Down,
    Left,
    Up,
}

impl Direction {
    fn turn_right(self) -> Direction {
        match self {
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Up => Direction::Right,
        }
    }
}

struct Spiral {
    size: usize,
}

impl Spiral {
    fn new(size: usize) -> Self {
        Spiral { size }
    }
    fn make(&self) -> Vec<Vec<i8>> {
        let mut position = Position::new(0, 0);
        let mut direction = Direction::Right;
        let mut grid: Vec<Vec<i8>> = vec![vec![0; self.size]; self.size];
        if position.is_valid(self.size) {
            grid[position.y as usize][position.x as usize] = 1;
            // println!("move to {:?}",position);
        }
        let mut direction_count = 0;
        loop {
            direction_count += 1;
            // println!("at {:?} -> {:?}",position, direction);
            let step1 = position.extend(&direction);
            // println!("step1 {:?}",step1);
            let step2 = step1.extend(&direction);
            // println!("step2 {:?}",step2);
            let not_step1 = !step1.is_valid(self.size);
            // println!("not_step1: {:?}",not_step1);
            let step2_occupied = step2.is_valid(self.size) && grid[step2.y as usize][step2.x as usize] == 1;
            // println!("step2_occupied: {:?}",step2_occupied);
            let turn_needed = !step1.is_valid(self.size) ||
                (step2.is_valid(self.size) && grid[step2.y as usize][step2.x as usize] == 1);
            if turn_needed {
                if direction_count == 1 {
                    // println!("Stopping after single step for direction.");
                    break;
                } else {
                    direction_count = 0;
                }
                // println!("turn_needed 1");
                direction = direction.turn_right();
                // println!("{:?}", direction);
                let step1 = position.extend(&direction);
                // println!("step1 {:?}",step1);
                let step2 = step1.extend(&direction);
                // println!("step2 {:?}",step2);
                let not_step1 = !step1.is_valid(self.size);
                // println!("not_step1: {:?}",not_step1);
                let step2_occupied = step2.is_valid(self.size) && grid[step2.y as usize][step2.x as usize] == 1;
                // println!("step2_occupied: {:?}",step2_occupied);
                let turn_needed = !step1.is_valid(self.size) ||
                    (step2.is_valid(self.size) && grid[step2.y as usize][step2.x as usize] == 1);
                if turn_needed {
                    // println!("turn_needed 2");
                    break;
                } else {
                    position = step1;
                    grid[position.y as usize][position.x as usize] = 1;
                }
            } else {
                position = step1;
                grid[position.y as usize][position.x as usize] = 1;
                // println!("move to {:?}",position);
            }
        }
        grid
    }
}

#[derive(Debug)]
struct Position {
    x: isize,
    y: isize,
}

impl Position {
    fn new(x: isize, y: isize) -> Self {
        Position { x, y }
    }
    fn extend(&self, direction: &Direction) -> Position {
        let x = self.x;
        let y = self.y;
        match direction {
            Direction::Right => Position::new(x+1,y),
            Direction::Down => Position::new(x,y+1),
            Direction::Left => Position::new(x-1,y),
            Direction::Up => Position::new(x,y-1),
        }
    }
    fn is_valid(&self, size: usize) -> bool {
        self.x >= 0 
        && self.x < size as isize
        && self.y >= 0
        && self.y < size as isize
    }
}