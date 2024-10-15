/// The languages used in the game
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub enum Language {
    #[default]
    English,
    Dutch,
}

/// From https://stackoverflow.com/a/32712140
impl std::fmt::Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        //write!(f, "{:?}", self)
        // or, alternatively:
        std::fmt::Debug::fmt(self, f)
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_compare_languages() {
        assert_eq!(Language::Dutch, Language::Dutch);
        assert_ne!(Language::Dutch, Language::English);
        assert_ne!(Language::English, Language::Dutch);
        assert_eq!(Language::English, Language::English);
    }
    #[test]
    fn test_can_convert_language_to_string() {
        assert_eq!(Language::Dutch.to_string(), "Dutch");
    }
}
