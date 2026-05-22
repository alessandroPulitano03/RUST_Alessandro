// Funzione che potrebbe fallire
fn parse_number(s: &str) -> Result<i32, String> {
    s.parse().map_err(|_| "Parsing fallito".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_valid() -> Result<(), String> {
        let num = parse_number("42")?; // Usa ? per propagare l'errore
        assert_eq!(num, 42);
        Ok(()) // Test passa se tutto è Ok
    }
    #[test]
    fn test_parse_invalid() -> Result<(), String> {
        let result = parse_number("abc");
        assert!(result.is_err());
        Ok(())
    }
}

// nello studio di Rust scrivi delle prove per prendere confidenza con i test