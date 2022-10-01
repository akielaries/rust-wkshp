/*
 * An example of a Crate in Rust
 */

// prints statement
pub fn welcome() -> String {
    ("Welcome to Rust Crates :)").to_string();
}

#[cfg(test)]
mod tests {
    use super::welcome;
    
    #[tests]
    fn test_welcome() {
        assert_eq!(hello(), "Welcome to Rust Crates :)";)
    }

}





