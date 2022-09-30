# bevy_mod_ui_texture_atlas_image
[![crates.io](https://img.shields.io/crates/v/bevy_mod_ui_texture_atlas_image)](https://crates.io/crates/bevy_mod_ui_texture_atlas_image)
[![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](https://github.com/ickshonpe/bevy_mod_ui_texture_atlas_image)
[![crates.io](https://img.shields.io/crates/d/bevy_mod_ui_texture_atlas_image)](https://crates.io/crates/bevy_mod_ui_texture_atlas_image)

https://github.com/ickshonpe/bevy_mod_ui_texture_atlas_image
Draw images from texture atlases with the Bevy UI.

Supports Bevy 0.8

Cargo dependency:
```toml
bevy_mod_ui_texture_atlas_image = "0.1"
```

## Details

To use add the ```UiTextureAtlasImagePlugin``` plugin to your Bevy App:

```rust
App::new()
    .add_plugins(DefaultPlugins)
    .add_plugin(UiTextureAtlasImagePlugin)
    /// rest of app
    .run()
```

Then you can spawn a ```TextureAtlasImageBundle``` to show images from texture atlases with the Bevy UI.

```TextureAtlasImageBundle``` is no different than ```ImageBundle``` except instead of a `UiImage` it takes a `UiTextureAtlasImage`.

```rust
commands
    .spawn_bundle(TextureAtlasImageBundle {
        image: UiTextureAtlasImage { 
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


