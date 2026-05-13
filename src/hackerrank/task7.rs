pub fn get_total_x(a: &[i32], b: &[i32]) -> i32 {
    let max_a = *a.iter().max().unwrap();
    let min_b = *b.iter().min().unwrap();
    
    let mut count = 0;
    
    for candidate in max_a..=min_b {
        let is_factor_of_all_a = a.iter().all(|&x| candidate % x == 0);
        let is_factor_of_all_b = b.iter().all(|&x| x % candidate == 0);
        
        if is_factor_of_all_a && is_factor_of_all_b {
            count += 1;
        }
    }
    
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let a = vec![2, 4];
        let b = vec![16, 32, 96];
        assert_eq!(get_total_x(&a, &b), 3);
    }

    #[test]
    fn test_single_element() {
        let a = vec![1];
        let b = vec![100];
        assert_eq!(get_total_x(&a, &b), 9);
    }

    #[test]
    fn test_no_numbers() {
        let a = vec![3, 5];
        let b = vec![7, 11];
        assert_eq!(get_total_x(&a, &b), 0);
    }
}