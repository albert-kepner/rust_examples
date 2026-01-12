fn main() {
    println!("Hello, world!");

    println!("Password123: {}", alphanumeric("Password123"));

    println!("_: {}", alphanumeric("_"));
}


fn alphanumeric(password: &str) -> bool {
    if password.is_empty() {
        return false;
    }
    for c in password.chars() {
        if !c.is_alphanumeric() {
            return false;
        }
    }
    return true;
}
