use bevy::prelude::*;
use bevy::render::texture::ImageSettings;
use bevy_mod_ui_texture_atlas_image::AtlasImageBundle;
use bevy_mod_ui_texture_atlas_image::UiAtlasImage;
use bevy_mod_ui_texture_atlas_image::UiAtlasImagePlugin;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    commands.spawn_bundle(Camera2dBundle::default());
    let image = asset_server.load("tileset_4x4.png");
    let texture_atlas = TextureAtlas::from_grid(image.clone(), 16. * Vec2::ONE, 4, 4);

    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            color: UiColor(Color::NONE),
            ..Default::default()
        })
        .with_children(|builder| {
            for index in [0, 5, 14] {
                builder.spawn_bundle(AtlasImageBundle {
                    atlas_image: UiAtlasImage {
                        atlas: texture_atlas_handle.clone(),
                        index,
                    },
                    color: UiColor(Color::rgba(1., 1., 1., 0.5)),
                    ..Default::default()
                });
            }
            builder.spawn_bundle(ImageBundle {
                image: image.into(),
                ..Default::default()
            });
        });
}

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            width: 600.,
            height: 128.,
            ..Default::default()
        })
        .insert_resource(ImageSettings::default_nearest())
        .add_plugins(DefaultPlugins)
        .add_plugin(UiAtlasImagePlugin)
        .add_startup_system(setup)
        .run();
}
