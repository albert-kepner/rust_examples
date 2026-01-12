fn main() {
    println!("Hello, world!");
    let square = &[
        vec![1,2,3],
        vec![4,5,6],
        vec![7,8,9],
    ];
    let result = snail(square);
    println!("{:?}", result); // Should print [1,2,3,6,9,8,7,4,5]
}


enum Direction {
    Right,
    Down,
    Left,
    Up,
}

struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn new(x: usize, y: usize) -> Self {
        Position { x, y }
    }
}

struct SnailIterator<'a> {
    matrix: &'a [Vec<i32>],
    direction: Direction,
    position: Position,
    visited: Vec<Vec<bool>>,
    total_elements: usize,
    visited_count: usize,
}

impl<'a> SnailIterator<'a> {
    fn new(matrix: &'a [Vec<i32>]) -> Self {
        let rows = matrix.len();
        let cols = if rows > 0 { matrix[0].len() } else { 0 };
        SnailIterator {
            matrix,
            direction: Direction::Right,
            position: Position::new(0, 0),
            visited: vec![vec![false; cols]; rows],
            total_elements: rows * cols,
            visited_count: 0,
        }
    }

    fn next_position(&self) -> Option<Position> {
        let (mut x, mut y) = (self.position.x, self.position.y);
        match self.direction {
            Direction::Right => x += 1,
            Direction::Down => y += 1,
            Direction::Left => {
                if x > 0 {
                    x -= 1;
                } else {
                    return None;
                }
            }
            Direction::Up => {
                if y > 0 {
                    y -= 1;
                } else {
                    return None;
                }
            }
        }
        if y < self.matrix.len() && x < self.matrix[0].len() && !self.visited[y][x] {
            Some(Position::new(x, y))
        } else {
            None
        }
    }

    fn next_step(&mut self) -> Option<i32> {
        if self.visited_count >= self.total_elements {
            return None;
        }

        let value = self.matrix[self.position.y][self.position.x];
        self.visited[self.position.y][self.position.x] = true;
        self.visited_count += 1;

        if let Some(next_pos) = self.next_position() {
            self.position = next_pos;
        } else {
            self.change_direction();
            if let Some(next_pos) = self.next_position() {
                self.position = next_pos;
            }
        }

        Some(value)
    }

    fn change_direction(&mut self) {
        self.direction = match self.direction {
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Up => Direction::Right,
        }
    }
      
}


fn snail(matrix: &[Vec<i32>]) -> Vec<i32> {
    let snail_iter = SnailIterator::new(matrix);
    let mut result = Vec::new();
    let mut iter = snail_iter;
    while let Some(value) = iter.next_step() {
        result.push(value);
    }
    result
}
