#[cfg(test)]
pub struct Assets {

}

#[cfg(test)]
impl Assets {
    #[cfg(test)]
    pub fn get_player_filename(&self, player_index: usize, singer_index: usize) -> String {
        let band = crate::band::Band{};
        let mut filename = band.get_name(player_index, singer_index);
        filename.push_str(".png");
        filename
    }
}

#[cfg(test)]
fn can_find_asset(filename: String) -> bool {
    let mut asset_filename = String::from("assets/");
    asset_filename.push_str(&filename.clone());
    std::path::Path::new(&asset_filename).exists()
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
    #[test]
    fn test_can_find_asset() {
        assert!(!can_find_asset(String::from("nonsense.png")));
        assert!(can_find_asset(String::from("K3OpEenRijMarthe.png")));
    }
}
