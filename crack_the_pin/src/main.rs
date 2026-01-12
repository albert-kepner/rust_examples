use md5;

pub fn crack(hash: &str) -> String {
    for i in 100000..199999 {
        let candidate = i.to_string()[1..].to_string(); // Get last 6 digits
        let digest = md5::compute(&candidate);
        let candidate_hash = format!("{:x}", digest);
        // println!("Trying {}: {}", candidate, candidate_hash);
        if candidate_hash == hash {
            return candidate.to_string();
        }
    }
    return "not found".to_string();
}

fn main() {
    
    let input = "12345";
    let digest = md5::compute(input);

    // Format as lowercase hex (e.g., "5eb63bbbe01eeed093cb22bb8f5acdc3")
    let hex = format!("{:x}", digest);
    println!("{}", hex);

    let decode = crack(&hex);
    println!("Cracked: {}", decode);

}

