fn main() {
    println!("Hello, world!");

    let mine =             &[
        vec![0, 0, 0, 0],
        vec![0, 0, 0, 0],
        vec![0, 0, 1, 0],
        vec![0, 0, 0, 0],
    ];

    let location = mine_location(mine);
    println!("Mine is located at: {:?}", location);
}

pub fn mine_location(field: &[Vec<u8>]) -> (usize, usize) {
    for i in 0..field.len() {
        let row = &field[i];
        for j in 0..row.len() {
            if row[j] == 1 {
                return (i, j);
            }
        }
    }
    (0, 0)
}
