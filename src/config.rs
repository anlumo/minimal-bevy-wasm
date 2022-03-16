use bevy::{app::PluginGroupBuilder, prelude::PluginGroup};

pub struct MyPlugins;

impl PluginGroup for MyPlugins {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group.add(bevy::log::LogPlugin::default());
        group.add(bevy::core::CorePlugin::default());
        group.add(bevy::transform::TransformPlugin::default());
        group.add(bevy::diagnostic::DiagnosticsPlugin::default());
        group.add(bevy::window::WindowPlugin {
            add_primary_window: false,
            exit_on_close: false,
        });
        group.add(bevy::asset::AssetPlugin::default());
        group.add(bevy::scene::ScenePlugin::default());
        group.add(crate::canvas::CanvasPlugin::default());
        group.add(bevy::render::RenderPlugin::default());
        group.add(bevy::core_pipeline::CorePipelinePlugin::default());
        group.add(bevy::sprite::SpritePlugin::default());
    }
}
