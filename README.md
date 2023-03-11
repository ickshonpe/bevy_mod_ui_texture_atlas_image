# bevy_mod_ui_texture_atlas_image
[![crates.io](https://img.shields.io/crates/v/bevy_mod_ui_texture_atlas_image)](https://crates.io/crates/bevy_mod_ui_texture_atlas_image)
[![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](https://github.com/ickshonpe/bevy_mod_ui_texture_atlas_image)
[![crates.io](https://img.shields.io/crates/d/bevy_mod_ui_texture_atlas_image)](https://crates.io/crates/bevy_mod_ui_texture_atlas_image)

Draw images from texture atlases with the Bevy UI.

![image](bevy_mod_ui_texture_atlas_image_long.png)

* Version 0.3 supports Bevy 0.10
* Version 0.2 supports Bevy 0.9
* Version 0.1 supports Bevy 0.8
#

## Details

To use this crate, add its dependency to your project's Cargo.toml:

```toml
bevy_mod_ui_texture_atlas_image = "0.3"
```

or use Cargo:

```
cargo add bevy_mod_ui_texture_atlas_image
```

Then add the ```UiAtlasImagePlugin``` plugin to your Bevy App:

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

Now you can spawn an `AtlasImageBundle` to draw images from a `TextureAtlas` with the Bevy UI.
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
The only difference between an `AtlasImageBundle` and an `ImageBundle` is that instead of an `image` field with type `UiImage`, it has an `atlas_image` field with type `UiAtlasImage`.

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



