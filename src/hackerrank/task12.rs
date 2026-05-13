pub fn birthday_cake_candles(candles: &[i32]) -> i32 {
    let max_height = *candles.iter().max().unwrap();
    candles.iter().filter(|&&height| height == max_height).count() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_input() {
        let candles = vec![3, 2, 1, 3];
        assert_eq!(birthday_cake_candles(&candles), 2);
    }

    #[test]
    fn test_all_same() {
        let candles = vec![4, 4, 4, 4];
        assert_eq!(birthday_cake_candles(&candles), 4);
    }
}