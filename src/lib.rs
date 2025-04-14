#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
    }

    #[test]
    fn test_sum() {
        let data = [10, 20, 30];
        assert_eq!(sum_slice(data.as_ptr(), data.len()), 60);
    }
}