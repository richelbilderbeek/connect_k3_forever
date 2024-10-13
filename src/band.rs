use crate::hair_color::HairColor;

/// Histroy of the band

pub struct Band {

}


/// The size of the band, the number of people that sing at the same time
pub fn get_band_size() -> usize {
    3
}

/// The number of singers with red hair
pub fn get_n_with_hair(hair_color: HairColor) -> usize {
    if hair_color == HairColor::Red {
        get_n_red_hair()
    } else if hair_color == HairColor::Black {
        get_n_black_hair()
    } else {
        assert_eq!(hair_color, HairColor::Blond);
        get_n_blond_hair()
    }
}


/// The number of singers with red hair
pub fn get_n_red_hair() -> usize {
    2
}

/// The number of singers with black hair
pub fn get_n_black_hair() -> usize {
    2
}

/// The number of singers with blond hair
pub fn get_n_blond_hair() -> usize {
    4
}

impl Band {

    /// @param hair_color the player's hair color
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
    pub fn get_name(&self, hair_color: HairColor, singer_index: usize) -> String {
        //assert!(player_index < get_band_size());
        assert!(singer_index < get_n_with_hair(hair_color.clone()));
        if hair_color == HairColor::Red { // Red hair
            assert!(singer_index < get_n_red_hair());
            if singer_index == 0 {
                return String::from("karen");
            }
            assert_eq!(singer_index, 1);
            return String::from("hanne")
        }
        if hair_color == HairColor::Black { // Black
            assert!(singer_index < get_n_black_hair());
            if singer_index == 0 {
                return String::from("kristel");
            }
            assert_eq!(singer_index, 1);
            return String::from("marthe")
        }
        assert_eq!(hair_color, HairColor::Blond);
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
    use crate::hair_color::HairColor;
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
    fn test_get_n_with_named_hair_color() {
        assert_eq!(get_n_red_hair(), 2);
        assert_eq!(get_n_black_hair(), 2);
        assert_eq!(get_n_blond_hair(), 4);
    }
    #[test]
    fn test_get_n_with_hair_color() {
        assert_eq!(get_n_red_hair(), get_n_with_hair(HairColor::Red));
        assert_eq!(get_n_black_hair(), get_n_with_hair(HairColor::Black));
        assert_eq!(get_n_blond_hair(), get_n_with_hair(HairColor::Blond));
    }

    #[test]
    fn test_can_get_name() {
        let band = Band{};
        assert!(!band.get_name(HairColor::Red, 0).is_empty());
    }
    #[test]
    fn test_names_are_correct() {
        let band = Band{};
        assert!(!band.get_name(HairColor::Red, 0).is_empty());
        assert_eq!(band.get_name(HairColor::Red, 0), "karen");
        assert_eq!(band.get_name(HairColor::Red, 1), "hanne");
        assert_eq!(band.get_name(HairColor::Black, 0), "kristel");
        assert_eq!(band.get_name(HairColor::Black, 1), "marthe");
        assert_eq!(band.get_name(HairColor::Blond, 0), "kathleen");
        assert_eq!(band.get_name(HairColor::Blond, 1), "josje");
        assert_eq!(band.get_name(HairColor::Blond, 2), "klaasje");
        assert_eq!(band.get_name(HairColor::Blond, 3), "julia");
    }
}
