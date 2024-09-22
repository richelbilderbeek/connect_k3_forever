/// Histroy of the band

pub struct Band {

}


/// The size of the band, the number of people that sing at the same time
#[cfg(test)]
pub fn get_band_size() -> i8 {
    3
}


/// The number of singers with red hair
#[cfg(test)]
pub fn get_n_red_hair() -> i8 {
    2
}

/// The number of singers with black hair
#[cfg(test)]
pub fn get_n_black_hair() -> i8 {
    2
}

/// The number of singers with blond hair
#[cfg(test)]
pub fn get_n_blond_hair() -> i8 {
    4
}

impl Band {

    #[cfg(test)]
    /// @param player_index the player's index:
    /// - 0: player 1, red hair
    /// - 1: player 2, black har
    /// - 2: player 3, blonde hair
    ///
    /// @param singer_index the index of the player.
    /// - For the singer with red hair:
    ///   - 0: Karen
    ///   - 1: Hanne
    /// - For the singer with black hair:
    ///   - 0: Kristel
    ///   - 1: Marthe
    /// - For the singer with blonde hair:
    ///   - 0: Kathleen
    ///   - 1: Josje
    ///   - 2: Klaasje
    ///   - 3: Julia
    pub fn get_name(&self, player_index: i8, singer_index: i8) -> String {
        assert!(player_index >= 0);
        assert!(player_index < get_band_size());
        assert!(singer_index >= 0);
        if player_index == 0 { // Red hair
            assert!(singer_index < get_n_red_hair());
            if singer_index == 0 {
                return String::from("karen");
            }
            assert_eq!(singer_index, 1);
            return String::from("hanne")
        }
        if player_index == 1 { // Black
            assert!(singer_index < get_n_black_hair());
            if singer_index == 0 {
                return String::from("kristel");
            }
            assert_eq!(singer_index, 1);
            return String::from("marthe")
        }
        assert_eq!(player_index, 2);
        assert!(singer_index < get_n_blond_hair());
        if singer_index == 0 {
            return String::from("kathleen");
        }
        else if singer_index == 1 {
            return String::from("josje");
        }
        else if singer_index == 2 {
            return String::from("klaasje");
        }
        assert_eq!(singer_index, 3);
        String::from("julia")
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_create() {
        let _assets = Band{};
    }

    #[test]
    fn test_get_band_size() {
        assert_eq!(get_band_size(), 3);
    }

    #[test]
    fn test_get_n_with_hair_color() {
        assert_eq!(get_n_red_hair(), 2);
        assert_eq!(get_n_black_hair(), 2);
        assert_eq!(get_n_blond_hair(), 4);
    }

    #[test]
    fn test_can_get_name() {
        let band = Band{};
        assert!(!band.get_name(0, 0).is_empty());
    }
    #[test]
    fn test_names_are_correct() {
        let band = Band{};
        assert!(!band.get_name(0, 0).is_empty());
        assert_eq!(band.get_name(0, 0), "karen");
        assert_eq!(band.get_name(0, 1), "hanne");
        assert_eq!(band.get_name(1, 0), "kristel");
        assert_eq!(band.get_name(1, 1), "marthe");
        assert_eq!(band.get_name(2, 0), "kathleen");
        assert_eq!(band.get_name(2, 1), "josje");
        assert_eq!(band.get_name(2, 2), "klaasje");
        assert_eq!(band.get_name(2, 3), "julia");
    }
}
