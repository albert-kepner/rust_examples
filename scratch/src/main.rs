fn main() {
    println!("Hello, world!");
    let size: usize = 10;
    
    // Create a 10×10 grid of isize values, all initialized to 1
    let mut grid: Vec<Vec<isize>> = vec![vec![1; size]; size];

    // Reassign the 3rd element (index 2) in the 1st row (index 0) to 3
    grid[0][2] = 3;

    println!("{:?}", grid[0]); // [1, 1, 3, 1, 1, 1, 1, 1, 1, 1]

}
