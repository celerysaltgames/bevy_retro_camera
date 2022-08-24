use bevy::ecs::bundle::Bundle;
use bevy::prelude::{Camera2dBundle, GlobalTransform, OrthographicProjection, Transform};
use bevy::render::camera::{Camera, CameraRenderGraph, DepthCalculation, ScalingMode};
use bevy::core_pipeline::core_2d::{Camera2d};
use bevy::render::primitives::Frustum;
use bevy::render::view::VisibleEntities;


/// 2D camera with easy controls for sizing the screen
#[derive(Bundle)]
pub struct RetroCameraBundle {
    pub camera_render_graph: CameraRenderGraph,
    pub projection: OrthographicProjection,
    pub visible_entities: VisibleEntities,
    pub frustum: Frustum,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub camera: Camera,
    pub camera_2d: Camera2d,
}

impl RetroCameraBundle {
    fn new(scale: f32, scaling_mode: ScalingMode) -> Self {
        // Modify the projection
        let orthographic_projection = OrthographicProjection {
            scale,
            scaling_mode,
            depth_calculation: DepthCalculation::ZDifference,
            ..Default::default()
        };

        // And copy the rest of the components from the default 2D camera
        let bundle = Camera2dBundle::default();
        Self {
            camera_render_graph: bundle.camera_render_graph,
            projection: orthographic_projection,
            visible_entities: bundle.visible_entities,
            frustum: bundle.frustum,
            transform: bundle.transform,
            global_transform: bundle.global_transform,
            camera: bundle.camera,
            camera_2d: bundle.camera_2d
        }
    }

    /// Create a camera with a fixed width in pixels and a height determined by the window aspect
    pub fn fixed_width(width: f32) -> Self {
        Self::new(width / 2.0, ScalingMode::FixedHorizontal(1.0))
    }

    /// Create a camera with a fixed height in pixels and a width determined by the window aspect
    pub fn fixed_height(height: f32) -> Self {
        Self::new(height / 2.0, ScalingMode::FixedVertical(1.0))
    }
}
