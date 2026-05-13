pub fn bon_appetit(bill: &[i32], k: usize, b: i32) -> String {
    let total: i32 = bill.iter().enumerate()
        .filter(|&(i, _)| i != k)
        .map(|(_, &price)| price)
        .sum();
    
    let anna_share = total / 2;
    
    if anna_share == b {
        "Bon Appetit".to_string()
    } else {
        (b - anna_share).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_overcharged() {
        let bill = vec![3, 10, 2, 9];
        assert_eq!(bon_appetit(&bill, 1, 12), "5");
    }

    #[test]
    fn test_correct() {
        let bill = vec![3, 10, 2, 9];
        assert_eq!(bon_appetit(&bill, 1, 7), "Bon Appetit");
    }

    #[test]
    fn test_first_item_not_eaten() {
        let bill = vec![5, 10, 15];
        assert_eq!(bon_appetit(&bill, 0, 12), "Bon Appetit");
    }

    #[test]
    fn test_last_item_not_eaten() {
        let bill = vec![1, 2, 3, 4];
        // Anna не їсть останній item (4), b = 3
        // Спільна сума = 1+2+3 = 6, Анна повинна заплатити 3
        // b = 3, тому "Bon Appetit"
        assert_eq!(bon_appetit(&bill, 3, 3), "Bon Appetit");
    }
}