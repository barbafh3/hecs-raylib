use hecs::World;
use raylib::{text::Font, RaylibHandle};

use super::scenes::{
    main_menu_scene::update_main_menu_scene, test_map_scene::update_test_map_scene, ActiveScene,
    Scene,
};

pub fn update_game(
    world: &mut World,
    raylib_handle: &mut RaylibHandle,
    font: &Font,
) -> Result<(), String> {
    let delta = raylib_handle.get_frame_time();

    let m_global_storage = world.query_mut::<&ActiveScene>().into_iter().nth(0);
    Ok(if let Some((_, active_scene)) = m_global_storage {
        return match active_scene.scene {
            Scene::TestMap => update_test_map_scene(world, font, delta),
            Scene::MainMenu => update_main_menu_scene(world, font, delta),
            Scene::TestMap2 => todo!(),
        };
    })
}
