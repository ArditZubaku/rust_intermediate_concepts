fn main() {
    println!("Hello, world!");
}

pub fn snuggle(bunnies: u128) -> u128 {
    bunnies * 8
}

// NOTE: This basically the compiler to compile this module only when we are running tests
#[cfg(test)]
mod test {
    use std::num::ParseIntError;

    use super::*;

    #[test]
    fn snuggling_bunnies_multiple() {
        assert_eq!(snuggle(2), 16);
    }

    // NOTE: Attributes stack on top of each other so the order doesn't matter
    #[test]
    #[should_panic]
    fn scared_bunny() {
        panic!("This would normally crash the program");
    }

    // NOTE: Tests can return results too
    #[test]
    fn bunny_result() -> Result<(), ParseIntError> {
        let num_bunnies = "four".parse::<u64>()?;
        assert_eq!(num_bunnies, 4);
        Ok(())
    }
}
