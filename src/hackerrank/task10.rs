use std::collections::HashMap;

pub fn sock_merchant(ar: &[i32]) -> i32 {
    let mut counts = HashMap::new();
    
    for &color in ar {
        *counts.entry(color).or_insert(0) += 1;
    }
    
    counts.values().map(|&count| count / 2).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let ar = vec![10, 20, 20, 10, 10, 30, 50, 10, 20];
        assert_eq!(sock_merchant(&ar), 3);
    }

    #[test]
    fn test_no_pairs() {
        let ar = vec![1, 2, 3, 4, 5];
        assert_eq!(sock_merchant(&ar), 0);
    }

    #[test]
    fn test_all_pairs() {
        let ar = vec![1, 1, 2, 2, 3, 3];
        assert_eq!(sock_merchant(&ar), 3);
    }

    #[test]
    fn test_single_pair() {
        let ar = vec![5, 5];
        assert_eq!(sock_merchant(&ar), 1);
    }
}