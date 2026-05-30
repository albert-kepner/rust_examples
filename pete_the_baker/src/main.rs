fn main() {
    println!("Hello, world!");
}

use std::collections::HashMap;

fn cakes(recipe: &HashMap<&str, u32>, available: &HashMap<&str, u32>) -> u32 {
    let mut n: u32 = 99999;
    for (ingredient, quantity) in recipe {
        let avail_count: u32 = match available.get(ingredient) {
            Some(v) => *v,
            None => 0,
        };
        
        let max_for_item = avail_count / quantity;
        if max_for_item < n {
            n = max_for_item;
        }
    }
    n
}
// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::cakes;
    use std::collections::HashMap;
    
    macro_rules! map {
        () => { HashMap::new() };
        ($($ingredient:ident : $amount:expr),+) => {{
            let mut map = HashMap::new();
            $( map.insert(stringify!($ingredient), $amount); )*
            map
        }};
    }
    
    fn test(recipe: &HashMap<&str, u32>, available: &HashMap<&str, u32>, expected: u32) {
        let actual = cakes(recipe, available);
        assert!(actual == expected, "Expected to bake {expected} cakes, but got {actual} cakes instead.\nAvailable ingredients:\n  {available:?}\nRecipe:\n  {recipe:?}\n\n");
    }

    #[test]
    fn test_example() {
        let recipe = map!(flour: 500, sugar: 200, eggs: 1);
        let available = map!(flour: 1200, sugar: 1200, eggs: 5, milk: 200);
        test(&recipe, &available, 2);
        
        let recipe = map!(apples: 3, flour: 300, sugar: 150, milk: 100, oil: 100);
        let available = map!(sugar: 500, flour: 2000, milk: 2000);
        test(&recipe, &available, 0);
    }
}
