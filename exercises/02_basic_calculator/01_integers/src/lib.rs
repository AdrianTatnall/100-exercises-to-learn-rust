fn compute(a: u32, b: u32) -> u32 {
    // TODO: change the line below to fix the compiler error and make the tests pass.
<<<<<<< Updated upstream
    let multiplier: u8 = 4;
    a + b * multiplier
=======
    a + b * 4u32
>>>>>>> Stashed changes
}

#[cfg(test)]
mod tests {
    use crate::compute;

    #[test]
    fn case() {
        assert_eq!(compute(1, 2), 9);
    }
}
