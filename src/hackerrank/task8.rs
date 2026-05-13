pub fn breaking_records(scores: &[i32]) -> Vec<i32> {
    let mut max_count = 0;
    let mut min_count = 0;
    let mut max_score = scores[0];
    let mut min_score = scores[0];
    
    for &score in scores.iter().skip(1) {
        if score > max_score {
            max_count += 1;
            max_score = score;
        } else if score < min_score {
            min_count += 1;
            min_score = score;
        }
    }
    
    vec![max_count, min_count]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_0() {
        let scores = vec![10, 5, 20, 20, 4, 5, 2, 25, 1];
        assert_eq!(breaking_records(&scores), vec![2, 4]);
    }

    #[test]
    fn test_sample_1() {
        let scores = vec![3, 4, 21, 36, 10, 28, 35, 5, 24, 42];
        assert_eq!(breaking_records(&scores), vec![4, 0]);
    }

    #[test]
    fn test_single_game() {
        let scores = vec![100];
        assert_eq!(breaking_records(&scores), vec![0, 0]);
    }

    #[test]
    fn test_all_same() {
        let scores = vec![5, 5, 5, 5];
        assert_eq!(breaking_records(&scores), vec![0, 0]);
    }
}