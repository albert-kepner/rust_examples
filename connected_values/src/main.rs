fn main() {
    println!("Hello, world!");
}

use std::collections::HashSet;

fn connected_values(arr: &[Vec<u8>], val: u8, coord: (usize, usize)) -> Vec<(usize, usize)> {
    let result: Vec<(usize, usize)> = Vec::new();
    if arr[coord.0][coord.1] != val {
        println!("No match to val, returning early...");
        return result;
    }
    result
}



// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
#[allow(unused_variables)]
mod tests {
    use super::connected_values;
    use itertools::Itertools;

    fn dotest(arr: &[Vec<u8>], val: u8, coord: (usize, usize), expected: &[(usize, usize)]) {
        try_equal(
            &connected_values(arr, val, coord)
                .iter()
                .sorted()
                .map(|x| *x)
                .collect::<Vec<(usize, usize)>>(),
            expected,
            val,
            coord,
        )
    }

    fn try_equal(
        connected: &Vec<(usize, usize)>,
        expected: &[(usize, usize)],
        val: u8,
        coord: (usize, usize),
    ) -> () {
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
