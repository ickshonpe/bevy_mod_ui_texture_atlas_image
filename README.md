# bevy_mod_ui_texture_atlas_image
[![crates.io](https://img.shields.io/crates/v/bevy_mod_ui_texture_atlas_image)](https://crates.io/crates/bevy_mod_ui_texture_atlas_image)
[![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](https://github.com/ickshonpe/bevy_mod_ui_texture_atlas_image)
[![crates.io](https://img.shields.io/crates/d/bevy_mod_ui_texture_atlas_image)](https://crates.io/crates/bevy_mod_ui_texture_atlas_image)

Draw images from texture atlases with the Bevy UI.

![image](bevy_mod_ui_texture_atlas_image_long.png)

* Versions 0.3 and 0.4 support Bevy 0.10
* Version 0.2 supports Bevy 0.9
* Version 0.1 supports Bevy 0.8
#

## Details

To use this crate, add its dependency to your project's `Cargo.toml`:

```toml
bevy_mod_ui_texture_atlas_image = "0.3"
```

or with Cargo:

```
cargo add bevy_mod_ui_texture_atlas_image
```

## Components
* `UiAtlasImage`

    The texture atlas image of the node.
* `ImageTint`

    The tint color of the image.

## Bundles
* `AtlasImageBundle`

    The bundle of components needed to display an image from a `TextureAtlas` with the Bevy UI.

## Plugin
The ```UiAtlasImagePlugin``` plugin must be added to your Bevy App:

```rust
use bevy_mod_ui_texture_atlas_image::*;

fn main () {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(UiAtlasImagePlugin)
        // ..rest of app
        .run()
}
```

Then you can spawn an `AtlasImageBundle` to draw images from a `TextureAtlas` with the Bevy UI:
```rust
commands
    .spawn(AtlasImageBundle {
        atlas_image: UiAtlasImage { 
            atlas: texture_atlas_handle.clone(),
            index: 5
        },
        ..Default::default()
    });
```
The differences between an `AtlasImageBundle` and an `ImageBundle` are that 
* Instead of a `UiImage` component, `AtlasImageBundle` has a `UiAtlasImage` component that sets the image displayed by the node.
* Instead of a `BackgroundColor` component, `AtlasImageBundle` has an `ImageTint` component that sets the color tint of the image.

#
### Examples

* Displaying a single image from a texture atlas:
    ``` 
    cargo --run --example minimal
    ```
* Displaying three tiles from a texture atlas grid alongside the atlas's source image:
    ``` 
    cargo --run --example tiles
    ```
* Displaying images from a texture atlas with an alpha channel.
    ```
    cargo --run --example alpha
    ```
