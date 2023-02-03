pub mod main_menu_scene;
pub mod test_map_scene;

#[derive(Debug, Clone)]
pub struct ActiveScene {
    pub scene: Scene,
}

#[derive(Debug, Clone)]
pub enum Scene {
    MainMenu,
    TestMap,
    TestMap2,
}
