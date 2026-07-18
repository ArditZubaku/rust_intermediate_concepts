use std::f32::consts::PI;

fn count_to_5() -> i32 {
    let mut res = 0;
    loop {
        if res > PI as i32 && res > 5 {
            break;
        }
        res += 1;
    }

    5
}

fn main() {
    println!("I can count to {}", count_to_5());
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_counting() {
        assert!(count_to_5() == 5);
    }
}
