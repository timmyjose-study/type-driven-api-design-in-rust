pub enum PrimaryColor {
    Red,
    Blue,
    Green,
}

pub fn color_to_rgb(colour: PrimaryColor) -> (u8, u8, u8) {
    match colour {
        PrimaryColor::Red => (255, 0, 0),
        PrimaryColor::Blue => (0, 255, 0),
        PrimaryColor::Green => (0, 0, 255),
    }
}

#[cfg(test)]
mod tests {
    use super::color_to_rgb;
    use super::PrimaryColor::*;

    #[test]
    fn test_rgb() {
        assert_eq!(color_to_rgb(Red), (255, 0, 0));
        assert_eq!(color_to_rgb(Blue), (0, 255, 0));
        assert_eq!(color_to_rgb(Green), (0, 0, 255));
    }
}
