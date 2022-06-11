pub fn color_to_rgb(colour: &str) -> Option<(u8, u8, u8)> {
    match colour {
        "Red" => Some((255, 0, 0)),
        "Blue" => Some((0, 255, 0)),
        "Green" => Some((0, 0, 255)),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rgb() {
        assert_eq!(color_to_rgb("Red"), Some((255, 0, 0)));
        assert_eq!(color_to_rgb("Blue"), Some((0, 255, 0)));
        assert_eq!(color_to_rgb("Green"), Some((0, 0, 255)));
    }

    #[test]
    fn test_rgb_neg() {
        assert!(color_to_rgb("red").is_none());
        assert!(color_to_rgb("Orange").is_none());
    }
}