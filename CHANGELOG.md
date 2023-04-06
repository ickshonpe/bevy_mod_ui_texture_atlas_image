# Version 0.4.1
* Fixed incorrect scaling of clipped texture atlas images.
* Added a new example `clipped`.

# Version 0.4.0
* Added `ImageTint` component (by Nionidh).
* Removed `BackgroundColor` from `AtlasImageBundle` and replaced it with `ImageTint` (by Nionidh).
* Query for `ImageTint` instead of `BackgroundColor` in the `extract_texture_atlas_image_uinodes` system (by Nionidh).
* Added a new example `alpha` (by Nionidh).
* Added `CHANGELOG.md`, this changelog.

# Version 0.3.0
* Bevy 0.10 support
* Added image flipping on the x and y-axes
* Added a `new` method on `UiAtlasImage` to create a `UiAtlasImage` from a `TextureAtlas` handle an index.

# Version 0.2.4
* Skip nodes during extraction if either of its dimensions has zero length.
* Moved the alpha check to before retrieving the atlas

# Version 0.2.2
* Added a `ZIndex` component to the `AtlasImageBundle`

# Version 0.2.0
* Bevy 0.9 support

# Version 0.1.0
* Bevy 0.8 support
