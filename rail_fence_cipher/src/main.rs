fn main() {
    println!("Hello, world!");
    let input = "WEAREDISCOVEREDFLEEATONCE";
    let num_rails = 3;
    let encoded = encode_rail_fence_cipher(input, num_rails);
    println!("Encoded: {}", encoded);
    let decoded = decode_rail_fence_cipher(&encoded, num_rails);
    println!("Decoded: {}", decoded);
}

struct RailIndex {
    num_rails: usize,
    rail: usize,
    direction: isize,
}

impl RailIndex {
    fn new(num_rails: usize) -> Self {
        RailIndex {
            num_rails,
            rail: 0,
            direction: 1,
        }
    }

    fn next(&mut self) -> usize {
        let current_rail = self.rail;
        if self.rail == 0 {
            self.direction = 1;
        } else if self.rail == self.num_rails - 1 {
            self.direction = -1;
        }
        self.rail = (self.rail as isize + self.direction) as usize;
        current_rail
    }
}


fn encode_rail_fence_cipher(text: &str, num_rails: usize) -> String {
    let mut rails: Vec<String> = vec![String::new(); num_rails];
    let mut rail_index = RailIndex::new(num_rails);
    for ch in text.chars() {
        let rail = rail_index.next();
        rails[rail].push(ch);
    }
    let mut result = String::new();
    for rail in rails {
        result.push_str(&rail);
    }
    result
}

fn decode_rail_fence_cipher(text: &str, num_rails: usize) -> String {
    let mut rail_index = RailIndex::new(num_rails);
    let mut rail_lengths = vec![0; num_rails];
    for _ in text.chars() {
        let rail = rail_index.next();
        rail_lengths[rail] += 1;
    }
    let mut rails: Vec<String> = vec![String::new(); num_rails];
    let mut current_index = 0;
    for i in 0..num_rails {
        let length = rail_lengths[i];
        rails[i] = text[current_index..current_index + length].to_string();
        current_index += length;
    }
    let mut result = String::new();
    let mut rail_index = RailIndex::new(num_rails);
    let mut rail_positions = vec![0; num_rails];
    for _ in text.chars() {
        let rail = rail_index.next();
        result.push(rails[rail].chars().nth(rail_positions[rail]).unwrap());
        rail_positions[rail] += 1;
    }
    result
}
