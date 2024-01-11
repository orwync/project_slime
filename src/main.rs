//! Shows how to render simple primitive shapes with a single color.

use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use level_1::Level1Plugin;
use crate::player::*;
use bevy_xpbd_2d::prelude::*;

mod player;
mod level_1;
mod collision;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, PhysicsPlugins::default()))
        .add_systems(Startup, setup)
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(PlayerPlugin)
        .add_plugins(Level1Plugin)
        .insert_resource(Gravity(Vec2::NEG_Y * 100.0))
        .insert_resource(PhysicsDebugConfig {
            aabb_color: Some(Color::WHITE),
            ..default()
        })
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
