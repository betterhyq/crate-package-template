fn add_two_numbers(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    println!("The sum of 1 and 2 is {}", add_two_numbers(1, 2));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_two_numbers() {
        assert_eq!(add_two_numbers(1, 2), 3);
    }
}
