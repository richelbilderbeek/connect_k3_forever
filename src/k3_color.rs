/// Colors used by K3
use bevy::color::Color;

/// Get the red color of the white 4-color rainbow dress
pub fn get_rainbow_dress_red() -> Color {
    Color::srgba(234.0 / 255.0,59.0 / 255.0, 54.0 / 255.0, 1.0)
}

/// Get the orange color of the white 4-color rainbow dress
#[cfg(test)]
pub fn get_rainbow_dress_orange() -> Color {
    Color::srgba(241.0 / 255.0,119.0 / 255.0, 46.0 / 255.0, 1.0)
}

/// Get the yellow color of the white 4-color rainbow dress
#[cfg(test)]
pub fn get_rainbow_dress_yellow() -> Color {
    Color::srgba(248.0 / 255.0,230.0 / 255.0, 32.0 / 255.0, 1.0)
}

/// Get the blue color of the white 4-color rainbow dress
#[cfg(test)]
pub fn get_rainbow_dress_blue() -> Color {
    Color::srgba(32.0 / 255.0,186.0 / 255.0, 222.0 / 255.0, 1.0)
}


mod tests {
    

    #[test]
    fn test_get_rainbow_dress_red() {
        let red = bevy::color::Color::srgba(234.0 / 255.0,59.0 / 255.0, 54.0 / 255.0, 1.0);
        assert_eq!(red, get_rainbow_dress_red());
    }
    #[test]
    fn test_get_rainbow_dress_orange() {
        let orange = bevy::color::Color::srgba(241.0 / 255.0,119.0 / 255.0, 46.0 / 255.0, 1.0);
        assert_eq!(orange, get_rainbow_dress_orange());
    }
    #[test]
    fn test_get_rainbow_dress_yellow() {
        let yellow = bevy::color::Color::srgba(248.0 / 255.0,230.0 / 255.0, 32.0 / 255.0, 1.0);
        assert_eq!(yellow, get_rainbow_dress_yellow());
    }

    #[test]
    fn test_get_rainbow_dress_blue() {
        let blue = bevy::color::Color::srgba(32.0 / 255.0,186.0 / 255.0, 222.0 / 255.0, 1.0);
        assert_eq!(blue, get_rainbow_dress_blue());
    }
}