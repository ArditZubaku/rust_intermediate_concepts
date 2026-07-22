pub fn sploosh(x: i32, y: i32, z: i32) -> i32 {
    match (x, y, z) {
        (x, _, _) if x < 0 => 99,
        (1, 2, 3) => 4,
        (5, 6, 7) => 3,
        (x, y, z) => x + y - z,
    }
}

pub fn splish(a: i32, b: i32) -> i32 {
    -a + 3 * b
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sploosh() {
        assert_eq!(sploosh(1, 2, 3), 4);
        assert_ne!(sploosh(5, 6, 7), 4);
        assert_eq!(sploosh(-1, 0, 0), 99);
    }

    #[test]
    fn test_splish() {
        assert!(splish(100, 10) < 0);
        assert!(splish(40, 20) > 0);
        assert!(splish(9, 3) == 0);
    }
}
