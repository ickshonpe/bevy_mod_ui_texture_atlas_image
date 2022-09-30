use bevy::prelude::*;
use bevy::render::Extract;
use bevy::render::RenderApp;
use bevy::render::RenderStage;
use bevy::ui::ExtractedUiNode;
use bevy::ui::ExtractedUiNodes;
use bevy::ui::FocusPolicy;
use bevy::ui::RenderUiSystem;
use bevy::ui::UiSystem;
use bevy::ui::widget::ImageMode;

#[derive(Component, Clone, Debug, Default, Reflect)]
#[reflect(Component, Default)]
pub struct UiTextureAtlasImage {
    pub atlas: Handle<TextureAtlas>,
    pub index: usize,
}

/// A UI node that is an image
#[derive(Bundle, Clone, Debug, Default)]
pub struct TextureAtlasImageBundle {
    /// Describes the size of the node
    pub node: Node,
    /// Describes the style including flexbox settings
    pub style: Style,
    /// Configures how the image should scale
    pub image_mode: ImageMode,
    /// The calculated size based on the given image
    pub calculated_size: CalculatedSize,
    /// The color of the node
    pub color: UiColor,
    /// The texture atlas image of the node
    pub image: UiTextureAtlasImage,
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
}

/// Updates calculated size of the node based on the image provided
fn texture_atlas_image_node_system(
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(&mut CalculatedSize, &UiTextureAtlasImage), With<ImageMode>>,
) {
    for (mut calculated_size, atlas_image) in &mut query {
        if let Some(atlas) = texture_atlases.get(&atlas_image.atlas) {
            let rect_size = atlas.textures[atlas_image.index].size();            
            let size = Size::new(rect_size.x, rect_size.y);

            // Update only if size has changed to avoid needless layout calculations
            if size != calculated_size.size {
                calculated_size.size = size;
            }
        }
    }
}

fn extract_texture_atlas_image_uinodes(
    mut extracted_uinodes: ResMut<ExtractedUiNodes>,
    images: Extract<Res<Assets<Image>>>,
    texture_atlases: Extract<Res<Assets<TextureAtlas>>>,
    uinode_query: Extract<
        Query<(
            &Node,
            &GlobalTransform,
            &UiColor,
            &UiTextureAtlasImage,
            &ComputedVisibility,
            Option<&CalculatedClip>,
        )>,
    >,
) {
    for (uinode, global_transform, color, atlas_image, visibility, clip) in uinode_query.iter() {
        if !visibility.is_visible() {
            continue;
        }
        if let Some(texture_atlas) = texture_atlases.get(&atlas_image.atlas) {
            let image = texture_atlas.texture.clone_weak();
            // Skip loading images
            if !images.contains(&image) {
                continue;
            }
            // Skip completely transparent nodes
            if color.0.a() == 0.0 {
                continue;
            }
            let rect = texture_atlas.textures[atlas_image.index];
            let scale = uinode.size / rect.size();
            let transform = global_transform.compute_matrix() * Mat4::from_scale(scale.extend(1.0));
            extracted_uinodes.uinodes.push(ExtractedUiNode {
                transform,
                color: color.0,
                rect,
                image,
                atlas_size: Some(texture_atlas.size),
                clip: clip.map(|clip| clip.clip),
            });
        }
    }
}

pub struct UiTextureAtlasImagePlugin;

impl Plugin for UiTextureAtlasImagePlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<UiTextureAtlasImage>()
            .add_system_to_stage(
                CoreStage::PostUpdate,
                texture_atlas_image_node_system.before(UiSystem::Flex),
            );

        let render_app = match app.get_sub_app_mut(RenderApp) {
            Ok(render_app) => render_app,
            Err(_) => return,
        };

        render_app
            .add_system_to_stage(
                RenderStage::Extract,
                extract_texture_atlas_image_uinodes
                .after(RenderUiSystem::ExtractNode)
            );
    }
}