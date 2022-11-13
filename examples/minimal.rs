use bevy::prelude::*;
use bevy_mod_ui_texture_atlas_image::*;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    commands.spawn(Camera2dBundle::default());
    let texture_atlas = TextureAtlas::from_grid(
        asset_server.load("numbered_grid_texture_atlas.png"),
        16. * Vec2::ONE,
        4,
        4,
        None,
        None,
    );
    let atlas = texture_atlases.add(texture_atlas);
    commands.spawn(AtlasImageBundle {
        atlas_image: UiAtlasImage { atlas, index: 0 },
        ..Default::default()
    });
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(UiAtlasImagePlugin)
        .add_startup_system(setup)
        .run();
}
