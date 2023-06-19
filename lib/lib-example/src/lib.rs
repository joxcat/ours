pub fn quoi() -> String {
    "feur".to_string()
}

#[cfg(test)]
mod tests {
    use crate::quoi;

    #[test]
    fn assert_quoi() {
        assert_eq!(quoi(), "feur")
    }
}
