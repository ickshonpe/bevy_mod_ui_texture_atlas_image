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
    commands
        .spawn(NodeBundle {
            style: Style {
                flex_basis: Val::Percent(100.),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                gap: Size::all(Val::Px(100.)),
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        size: Size::all(Val::Px(400.)),
                        overflow: Overflow::Hidden,
                        ..Default::default()
                    },
                    background_color: Color::RED.into(),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent.spawn(AtlasImageBundle {
                        style: Style {
                            size: Size::all(Val::Px(400.)),
                            position: UiRect {
                                bottom: Val::Px(100.),
                                right: Val::Px(100.),
                                ..default()
                            },
                            ..Default::default()
                        },
                        color: TintColor(Color::WHITE),
                        atlas_image: UiAtlasImage::new(atlas.clone(), 5),
                        ..Default::default()
                    });
                });

            parent
                .spawn(NodeBundle {
                    style: Style {
                        size: Size::all(Val::Px(400.)),
                        overflow: Overflow::Hidden,
                        ..Default::default()
                    },
                    background_color: Color::GREEN.into(),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent.spawn(AtlasImageBundle {
                        style: Style {
                            size: Size::all(Val::Px(400.)),
                            position: UiRect {
                                top: Val::Px(100.),
                                left: Val::Px(100.),
                                ..default()
                            },
                            ..Default::default()
                        },
                        color: TintColor(Color::WHITE),
                        atlas_image: UiAtlasImage::new(atlas, 8),
                        ..Default::default()
                    });
                });
        });
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(UiAtlasImagePlugin)
        .add_startup_system(setup)
        .run();
}
