use bevy::prelude::*;
use bevy_mod_ui_texture_atlas_image::AtlasImageBundle;
use bevy_mod_ui_texture_atlas_image::UiAtlasImage;
use bevy_mod_ui_texture_atlas_image::UiAtlasImagePlugin;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    commands.spawn(Camera2dBundle::default());

    // The source image for the atlas has tiles labelled with numbers that correspond
    // to their respective indices assigned by TextureAtlas::from_grid.
    let image = asset_server.load("numbered_grid_texture_atlas.png");
    let texture_atlas = TextureAtlas::from_grid(image.clone(), 16. * Vec2::ONE, 4, 4, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    // Root ui node that fills the window.
    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            background_color: BackgroundColor(Color::NONE),
            ..Default::default()
        })
        .with_children(|builder| {
            // Spawn UiAtlasImage ui nodes for the tiles numbered 0, 5, and 14.
            for index in [0, 5, 14] {
                builder.spawn(AtlasImageBundle {
                    atlas_image: UiAtlasImage {
                        atlas: texture_atlas_handle.clone(),
                        index,
                    },
                    ..Default::default()
                });
            }
            // Spawn an ordinary Image ui node displaying the source atlas image.
            builder.spawn(ImageBundle {
                image: image.into(),
                ..Default::default()
            });
        });
}

fn main() {
    App::new()
        // Change the default image filtering to nearest so the images are sharp.
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    window: WindowDescriptor {
                        width: 600.,
                        height: 128.,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugin(UiAtlasImagePlugin)
        .add_startup_system(setup)
        .run();
}
