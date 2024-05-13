fn add1(x: i32, y: i32) -> i32 {
    x + y
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_add1() {
        assert_eq!(add1(1, 2), 3);
    }
}
