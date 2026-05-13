pub fn migratory_birds(arr: &[i32]) -> i32 {
    let mut counts = [0; 5];
    
    for &bird in arr {
        counts[(bird - 1) as usize] += 1;
    }
    
    let mut max_count = 0;
    let mut max_type = 0;
    
    for (i, &count) in counts.iter().enumerate() {
        if count > max_count {
            max_count = count;
            max_type = i + 1;
        }
    }
    
    max_type as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_0() {
        let arr = vec![1, 4, 4, 4, 5, 3];
        assert_eq!(migratory_birds(&arr), 4);
    }

    #[test]
    fn test_sample_1() {
        let arr = vec![1, 2, 3, 4, 5, 4, 3, 2, 1, 3, 4];
        assert_eq!(migratory_birds(&arr), 3);
    }

    #[test]
    fn test_all_same() {
        let arr = vec![2, 2, 2, 2];
        assert_eq!(migratory_birds(&arr), 2);
    }

    #[test]
    fn test_tie_lower_wins() {
        let arr = vec![1, 1, 2, 2, 3];
        assert_eq!(migratory_birds(&arr), 1);
    }
}