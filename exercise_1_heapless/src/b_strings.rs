use heapless::{String, Vec};

/// Create a string with a heart as content
///
/// - [x] Find UTF-8 encoding of the heart emoji
/// - [ ] Create a string with the sparkle heart
pub fn give_me_a_heart() -> String<4> {
    // To make a heart you need to use these bytes
    let sparkle_heart = Vec::from_slice(&[240, 159, 146, 150]).unwrap();
    String::from_utf8(sparkle_heart).unwrap()
}

/// Create a string with a pizza and a heart
///
/// - [x] Create a pizza string
/// - [x] Create a heart string
/// - [ ] Create a string with pizza and sparkle heart combined
pub fn give_me_a_pizza_heart() -> String<12> {
    use core::fmt::Write;

    // Combine this pizza with some love!
    let pizza: &str = "🍕️";
    let heart = give_me_a_heart();

    // We have an empty string here, can you append both together? Hint: You can use write! for this
    // The format of write! is just like format!
    let mut pizza_heart: String<12> = String::new();
    write!(pizza_heart, "{}{}", pizza, heart).unwrap();
    pizza_heart
}

#[cfg(test)]
mod tests {
    use crate::b_strings::{give_me_a_heart, give_me_a_pizza_heart};

    #[test]
    fn test_give_me_a_heart() {
        let sparkle_heart = give_me_a_heart();
        assert_eq!("💖", sparkle_heart);
    }

    #[test]
    fn test_give_me_a_pizza_heart() {
        let sparkling_pizza_heart = give_me_a_pizza_heart();
        assert_eq!("🍕️💖", sparkling_pizza_heart);
    }
}
