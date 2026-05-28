pub mod models;

#[cfg(test)]
mod tests {
    use crate::models::two_sum::two_sum; // 👈 models se import
    #[test]
    fn test_two_sum() {
        assert_eq!(two_sum(2, 3), 5);
    }
}