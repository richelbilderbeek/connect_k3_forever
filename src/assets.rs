struct Assets {

}

impl Assets {
    #[cfg(test)]
    fn get_player_filename(&self, player_index: i8, singer_index: i8) -> String {
        let band = crate::band::Band{};
        let mut filename = band.get_name(player_index, singer_index);
        filename.push_str(".png");
        return filename
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_create() {
        let _assets = Assets{};
    }
    #[test]
    fn test_can_get_a_player_filename() {
        let assets = Assets{};
        assert!(!assets.get_player_filename(0, 0).is_empty());
    }
    #[test]
    fn test_can_get_correct_player_filename() {
        let assets = Assets{};
        assert_eq!(assets.get_player_filename(0, 0), "karen.png");
    }
}
