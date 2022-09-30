use bevy::prelude::*;
use bevy::render::Extract;
use bevy::render::RenderApp;
use bevy::render::RenderStage;
use bevy::ui::widget::ImageMode;
use bevy::ui::ExtractedUiNode;
use bevy::ui::ExtractedUiNodes;
use bevy::ui::FocusPolicy;
use bevy::ui::RenderUiSystem;
use bevy::ui::UiSystem;

#[derive(Component, Clone, Debug, Default, Reflect)]
#[reflect(Component, Default)]
pub struct UiAtlasImage {
    /// asset handle of the texture atlas
    pub atlas: Handle<TextureAtlas>,
    /// index of the image in the texture atlas
    pub index: usize,
}

/// A UI node that is an image
#[derive(Bundle, Clone, Debug, Default)]
pub struct AtlasImageBundle {
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
}

fn texture_atlas_image_node_system(
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(&mut CalculatedSize, &UiAtlasImage), With<ImageMode>>,
) {
    for (mut calculated_size, atlas_image) in &mut query {
        if let Some(atlas) = texture_atlases.get(&atlas_image.atlas) {
            let rect_size = atlas.textures[atlas_image.index].size();
            let size = Size::new(rect_size.x, rect_size.y);
            if size != calculated_size.size {
                calculated_size.size = size;
            }
        }
    }
}

#[allow(clippy::type_complexity)]
fn extract_texture_atlas_image_uinodes(
    mut extracted_uinodes: ResMut<ExtractedUiNodes>,
    images: Extract<Res<Assets<Image>>>,
    texture_atlases: Extract<Res<Assets<TextureAtlas>>>,
    uinode_query: Extract<
        Query<(
            &Node,
            &GlobalTransform,
            &UiColor,
            &UiAtlasImage,
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
            if !images.contains(&image) || color.0.a() == 0.0 {
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

pub struct UiAtlasImagePlugin;

impl Plugin for UiAtlasImagePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<UiAtlasImage>().add_system_to_stage(
            CoreStage::PostUpdate,
            texture_atlas_image_node_system.before(UiSystem::Flex),
        );

        let render_app = match app.get_sub_app_mut(RenderApp) {
            Ok(render_app) => render_app,
            Err(_) => return,
        };

        render_app.add_system_to_stage(
            RenderStage::Extract,
            extract_texture_atlas_image_uinodes.after(RenderUiSystem::ExtractNode),
        );
    }
}
