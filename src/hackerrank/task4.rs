pub fn grading_students(grades: &[i32]) -> Vec<i32> {
    grades
        .iter()
        .map(|&grade| {
            if grade < 38 {
                grade
            } else {
                let next_multiple = ((grade / 5) + 1) * 5;
                if next_multiple - grade < 3 {
                    next_multiple
                } else {
                    grade
                }
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grading() {
        let grades = vec![73, 67, 38, 33];
        let result = grading_students(&grades);
        assert_eq!(result, vec![75, 67, 40, 33]);
    }

    #[test]
    fn test_all_below_38() {
        let grades = vec![37, 36, 35, 30];
        let result = grading_students(&grades);
        assert_eq!(result, vec![37, 36, 35, 30]);
    }

    // Цей тест відповідає реальним правилам HackerRank
    #[test]
    fn test_hackerrank_rules() {
        let grades = vec![84, 83, 82, 81, 80];
        let result = grading_students(&grades);
        // 84 -> 85 (різниця 1), 83 -> 85 (різниця 2), 82 -> 82 (різниця 3)
        assert_eq!(result, vec![85, 85, 82, 81, 80]);
    }
}