use bevy::math::vec2;
use bevy::prelude::*;
use bevy_mod_ui_texture_atlas_image::*;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    commands.spawn_bundle(Camera2dBundle::default());
    let texture_atlas = TextureAtlas::from_grid(
        asset_server.load("numbered_grid_texture_atlas.png"),
        16. * Vec2::ONE,
        4,
        4,
    );
    let atlas =  texture_atlases.add(texture_atlas);
    //println!("{:#?}", texture_atlas.textures);
    commands.spawn_bundle(AtlasImageBundle {
        atlas_image: UiAtlasImage {
            atlas: atlas.clone(),
            index: 0,
        },
        ..Default::default()
    });

    if let Some(breakout_atlas) = texture_atlases.get_mut(&atlas) {
        let red_ball_index = breakout_atlas.add_texture(bevy::sprite::Rect { min: vec2(32., 0.), max: vec2(64.0, 32.0) });
        commands.spawn_bundle(SpriteSheetBundle {
            sprite: TextureAtlasSprite::new(red_ball_index),
            texture_atlas: atlas.clone(),
            ..Default::default()
        });
      }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(UiAtlasImagePlugin)
        .add_startup_system(setup)
        .run();
}
