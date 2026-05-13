pub fn staircase(n: i32) {
    for i in 1..=n {
        let spaces = (n - i) as usize;
        let hashes = i as usize;
        println!("{}{}", " ".repeat(spaces), "#".repeat(hashes));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_staircase_4() {
        staircase(4);
    }

    #[test]
    fn test_staircase_1() {
        staircase(1);
    }
}