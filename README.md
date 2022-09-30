# bevy_mod_ui_texture_atlas_image
[![crates.io](https://img.shields.io/crates/v/bevy_mod_ui_texture_atlas_image)](https://crates.io/crates/bevy_mod_ui_texture_atlas_image)
[![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](https://github.com/ickshonpe/bevy_mod_ui_texture_atlas_image)
[![crates.io](https://img.shields.io/crates/d/bevy_mod_ui_texture_atlas_image)](https://crates.io/crates/bevy_mod_ui_texture_atlas_image)

Draw images from texture atlases with Bevy UI.

![image](bevy_mod_ui_texture_atlas_image.png)

Supports Bevy 0.8

Cargo dependency:
```toml
bevy_mod_ui_texture_atlas_image = "0.1"
```

## Details

To use add the ```UiAtlasImagePlugin``` plugin to your Bevy App:

```rust
App::new()
    .add_plugins(DefaultPlugins)
    .add_plugin(UiAtlasImagePlugin)
    // ..rest of app
    .run()
```

Now you can spawn an `AtlasImageBundle` to draw images from a `TextureAtlas` with the Bevy UI.

The only difference between `AtlasImageBundle` and `ImageBundle` is that instead of an `image` field with type `UiImage`, it has an `atlas_image` field with type `UiAtlasImage`.

```rust
commands
    .spawn_bundle(AtlasImageBundle {
        atlas_image: UiAtlasImage { 
            atlas: texture_atlas_handle.clone(),
            index: 5
        },
        ..Default::default()
    });
```
#
### Minimal Example

``` 
cargo --run --example minimal
```


