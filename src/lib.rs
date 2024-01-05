pub mod akai;
pub mod superflake;

#[cfg(test)]
mod tests {
    #[test]
    fn generate() {
        let prefix = crate::akai::AkaiPrefix {
            content: "user".to_string(),
            allows_lowercase: true
        };
    
        let akai = crate::akai::Akai::new(crate::akai::AkaiOptions {
            prefix,
            custom_bytes: Some(8),
            custom_timestamp: None
        });
    
        match akai.generate() {
            Ok(id) => println!("{}", id),
            Err(error) => eprintln!("Error: {}", error)
        }
    }
}