mod greet_plugin;
mod goku;

use bevy::prelude::*;
use greet_plugin::GreetPlugin;
use goku::GokuPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        //.add_plugin(GreetPlugin)
        .add_startup_system(setup)
        .add_plugin(GokuPlugin)
        .add_startup_system(_setup_dot)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

}

fn _setup_dot(mut commands: Commands, asset_server: Res<AssetServer>, mut assets: ResMut<Assets<TextureAtlas>>) {
    let texture = asset_server.load("Goku.png");

    let atlas = TextureAtlas::from_grid(texture, Vec2 { x: 95.0, y: 140.0 }, 1, 1, Some(Vec2::splat(0.0)), Some(Vec2 { x: 12.0, y: 0.0 }));

    let atlas_handle = assets.add(atlas);

    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: atlas_handle,
            transform: Transform::from_translation(Vec3::splat(300.0)),
            ..default()
        },
    ));
}
