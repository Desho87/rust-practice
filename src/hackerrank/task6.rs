pub fn kangaroo(x1: i32, v1: i32, x2: i32, v2: i32) -> String {
    if v1 == v2 {
        if x1 == x2 {
            return "YES".to_string();
        } else {
            return "NO".to_string();
        }
    }
    
    let diff_x = x2 - x1;
    let diff_v = v1 - v2;
    
    if diff_v != 0 && diff_x % diff_v == 0 && diff_x / diff_v > 0 {
        "YES".to_string()
    } else {
        "NO".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_yes() {
        assert_eq!(kangaroo(0, 3, 4, 2), "YES");
    }

    #[test]
    fn test_sample_no() {
        assert_eq!(kangaroo(0, 2, 5, 3), "NO");
    }

    #[test]
    fn test_same_position_same_speed() {
        assert_eq!(kangaroo(5, 2, 5, 2), "YES");
    }

    #[test]
    fn test_different_position_same_speed() {
        assert_eq!(kangaroo(0, 2, 5, 2), "NO");
    }
}