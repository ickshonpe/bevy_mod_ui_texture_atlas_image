use bevy::prelude::*;
use bevy::render::Extract;
use bevy::render::RenderApp;
use bevy::ui::ExtractedUiNode;
use bevy::ui::ExtractedUiNodes;
use bevy::ui::FocusPolicy;
use bevy::ui::RenderUiSystem;
use bevy::ui::UiStack;
use bevy::ui::UiSystem;

/// A component that represents an image from a `TextureAtlas`.
#[derive(Component, Clone, Debug, Default, Reflect)]
#[reflect(Component, Default)]
pub struct UiAtlasImage {
    /// assets handle of the texture atlas
    pub atlas: Handle<TextureAtlas>,
    /// index of the image in the texture atlas
    pub index: usize,
    /// Whether the image should be flipped along its x-axis
    pub flip_x: bool,
    /// Whether the image should be flipped along its y-axis
    pub flip_y: bool,
}

impl UiAtlasImage {
    /// Creates a new `UiAtlasImage` from a `TextureAtlas` handle and an index.
    pub fn new(atlas: Handle<TextureAtlas>, index: usize) -> Self {
        Self {
            atlas,
            index,
            flip_x: false,
            flip_y: false,
        }
    }
}

/// A UI node that is an image from a texture atlas
#[derive(Bundle, Clone, Debug, Default)]
pub struct AtlasImageBundle {
    /// Describes the size of the node
    pub node: Node,
    /// Describes the style including flexbox settings
    pub style: Style,
    /// The calculated size based on the given image
    pub calculated_size: CalculatedSize,
    /// The color of the image
    pub color: BackgroundColor,
    /// The texture atlas image of the node
    pub atlas_image: UiAtlasImage,
    /// Whether this node should block interaction with lower nodes
    pub focus_policy: FocusPolicy,
    /// The transform of the node
    pub transform: Transform,
    /// The global transform of the node
    pub global_transform: GlobalTransform,
    /// Describes the visibility properties of the node
    pub visibility: Visibility,
    /// Algorithmically-computed indication of whether an entity is visible and should be extracted for rendering
    pub computed_visibility: ComputedVisibility,
    /// Indicates the depth at which the node should appear in the UI
    pub z_index: ZIndex,
}

fn texture_atlas_image_node_system(
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(&mut CalculatedSize, &UiAtlasImage)>,
) {
    for (mut calculated_size, atlas_image) in &mut query {
        if let Some(atlas) = texture_atlases.get(&atlas_image.atlas) {
            let size = atlas.textures[atlas_image.index].size();
            if size != calculated_size.size {
                calculated_size.size = size;
                calculated_size.preserve_aspect_ratio = true;
            }
        }
    }
}

#[allow(clippy::type_complexity)]
fn extract_texture_atlas_image_uinodes(
    mut extracted_uinodes: ResMut<ExtractedUiNodes>,
    images: Extract<Res<Assets<Image>>>,
    texture_atlases: Extract<Res<Assets<TextureAtlas>>>,
    ui_stack: Extract<Res<UiStack>>,
    uinode_query: Extract<
        Query<(
            &Node,
            &GlobalTransform,
            &BackgroundColor,
            &UiAtlasImage,
            &ComputedVisibility,
            Option<&CalculatedClip>,
        )>,
    >,
) {
    for (stack_index, entity) in ui_stack.uinodes.iter().enumerate() {
        if let Ok((uinode, global_transform, color, atlas_image, visibility, clip)) =
            uinode_query.get(*entity)
        {
            if !visibility.is_visible()
                || uinode.size().x == 0.
                || uinode.size().y == 0.
                || color.0.a() == 0.0
            {
                continue;
            }

            if let Some(texture_atlas) = texture_atlases.get(&atlas_image.atlas) {
                let image = texture_atlas.texture.clone_weak();
                if !images.contains(&image) {
                    continue;
                }
                let rect = texture_atlas.textures[atlas_image.index];
                let scale = uinode.size() / rect.size();
                let transform =
                    global_transform.compute_matrix() * Mat4::from_scale(scale.extend(1.0));
                extracted_uinodes.uinodes.push(ExtractedUiNode {
                    stack_index,
                    transform,
                    color: color.0,
                    rect,
                    image,
                    atlas_size: Some(texture_atlas.size),
                    clip: clip.map(|clip| clip.clip),
                    flip_x: atlas_image.flip_x,
                    flip_y: atlas_image.flip_y,
                });
            }
        }
    }
}
pub struct UiAtlasImagePlugin;

impl Plugin for UiAtlasImagePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<UiAtlasImage>().add_system(
            texture_atlas_image_node_system
                .before(UiSystem::Flex)
                .in_base_set(CoreSet::PostUpdate),
        );

        let render_app = match app.get_sub_app_mut(RenderApp) {
            Ok(render_app) => render_app,
            Err(_) => return,
        };

        render_app.add_system(
            extract_texture_atlas_image_uinodes
                .after(RenderUiSystem::ExtractNode)
                .in_schedule(ExtractSchedule),
        );
    }
}
