pub fn divisible_sum_pairs(k: i32, ar: &[i32]) -> i32 {
    let mut count = 0;
    let n = ar.len();
    
    for i in 0..n {
        for j in i + 1..n {
            if (ar[i] + ar[j]) % k == 0 {
                count += 1;
            }
        }
    }
    
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let ar = vec![1, 3, 2, 6, 1, 2];
        assert_eq!(divisible_sum_pairs(3, &ar), 5);
    }

    #[test]
    fn test_small_array() {
        let ar = vec![1, 2, 3, 4];
        assert_eq!(divisible_sum_pairs(2, &ar), 2);
    }

    #[test]
    fn test_no_pairs() {
        let ar = vec![1, 4, 7, 10];
        assert_eq!(divisible_sum_pairs(3, &ar), 0);
    }

    #[test]
    fn test_all_pairs() {
        let ar = vec![2, 4, 6, 8];
        assert_eq!(divisible_sum_pairs(2, &ar), 6);
    }
}