use crate::hair_color::HairColor;

pub struct GameAssets {

}

impl GameAssets {

    pub fn new() -> GameAssets {
        GameAssets{}
    }

    pub fn get_player_filename(&self, hair_color: HairColor, singer_index: usize) -> String {
        let band = crate::band::Band{};
        let mut filename = band.get_name(hair_color, singer_index);
        filename.push_str(".png");
        filename
    }
}

#[cfg(test)]
fn can_find_asset(filename: String) -> bool {
    /*
    From s33n on the Bevy Discord server

    use std::path::PathBuf;
    use bevy::asset::io::AssetSourceId;
    use bevy::prelude::*;
    use bevy::tasks::{AsyncComputeTaskPool, block_on, futures_lite::StreamExt};

    fn main() {
        let mut app = App::new();
        app.add_plugins((AssetPlugin::default(), TaskPoolPlugin::default()));
        let server = app.world().resource::<AssetServer>();
        println!("1: {}", server.can_find_file(PathBuf::from("lvl1.png")));
        println!("2: {}", server.can_find_file(PathBuf::from("lvl2.png")));
    }

    pub trait AssetServerFindExt {
        fn can_find_file(&self, path: PathBuf) -> bool;
    }

    impl AssetServerFindExt for AssetServer {
        fn can_find_file(&self, path: PathBuf) -> bool {
            let thread_pool = AsyncComputeTaskPool::get();

            let server = self.clone();
            let task = thread_pool.spawn(async move {
                let Some(parent) = path.parent() else { return false };
                let Some(file) = path.file_name() else { return false };

                let source = server.get_source(AssetSourceId::Default).unwrap();
                let reader = source.reader();
                let Ok(mut stream) = reader.read_directory(parent).await else { return false };
                while let Some(path) = stream.next().await {
                    if path == file {
                        return true
                    }
                }
                false
            });

            block_on(task)
        }
    }
     */
    // Approach similar to Pnoenix from the Bevy Discord server
    let mut asset_filename = String::from("assets/");
    asset_filename.push_str(&filename.clone());
    std::path::Path::new(&asset_filename).exists()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_create() {
        let _assets = GameAssets::new();
    }
    #[test]
    fn test_can_get_a_player_filename() {
        let assets = GameAssets::new();
        assert!(!assets.get_player_filename(HairColor::Red, 0).is_empty());
    }
    #[test]
    fn test_can_get_correct_player_filename() {
        let assets = GameAssets::new();
        assert_eq!(assets.get_player_filename(HairColor::Red, 0), "karen.png");
    }
    #[test]
    fn test_can_find_asset() {
        assert!(!can_find_asset(String::from("nonsense.png")));
        assert!(can_find_asset(String::from("marthe.png")));
    }
    #[test]
    fn test_can_find_all_band_assets() {
        let assets = GameAssets::new();
        assert!(can_find_asset(assets.get_player_filename(HairColor::Red, 0)));
        assert!(can_find_asset(assets.get_player_filename(HairColor::Red, 1)));
        assert!(can_find_asset(assets.get_player_filename(HairColor::Black, 0)));
        assert!(can_find_asset(assets.get_player_filename(HairColor::Black, 1)));
        assert!(can_find_asset(assets.get_player_filename(HairColor::Blond, 0)));
        assert!(can_find_asset(assets.get_player_filename(HairColor::Blond, 1)));
        assert!(can_find_asset(assets.get_player_filename(HairColor::Blond, 2)));
        assert!(can_find_asset(assets.get_player_filename(HairColor::Blond, 3)));
    }
}
