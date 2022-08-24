use bevy::prelude::{App, Msaa, Plugin};
use bevy::render::texture::{ImageSampler, ImageSettings};


pub struct RetroCameraBundle;

impl Plugin for RetroCameraPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Msaa { samples: 1 });  // Disable anti-aliasing
        app.insert_resource(ImageSettings { default_sampler: ImageSampler::nearest_descriptor() });
    }
}
