use bevy::prelude::*;
use crate::movable::Movable;

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(Timer);

#[derive(Default)]
pub enum PlayerState {
    #[default]
    Idle,
    MoveAhead,
    MoveBack,
    Punch,
}


#[derive(Default, Component)]
pub struct Player {
    state: PlayerState,
}

pub struct GokuPlugin;

impl Plugin for GokuPlugin {
    fn build(&self, app: &mut App) {
        app
           .add_startup_system(spawn_goku)
           .add_system(animate_goku);
    }
}

fn spawn_goku(mut commands: Commands, asset_server: Res<AssetServer>, mut assets: ResMut<Assets<TextureAtlas>>) {
    let texture = asset_server.load("Goku.png");

    let atlas = TextureAtlas::from_grid(texture, Vec2 { x: 97.0, y: 120.0 }, 4, 1, Some(Vec2::splat(0.0)), Some(Vec2 { x: 10.0, y: 10.0 }));

    let atlas_handle = assets.add(atlas);

    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: atlas_handle,
            transform: Transform::from_translation(Vec3::splat(0.0)),
            ..default()
        },
        AnimationTimer(Timer::from_seconds(0.5, TimerMode::Repeating)),
        Movable,
        Player::default(),
    ));
}

fn animate_goku(
    mut query: Query<(&Transform, &mut AnimationTimer, &mut TextureAtlasSprite, &Handle<TextureAtlas>)>,
    time: Res<Time>,
    textures: Res<Assets<TextureAtlas>>
) {
    for (_, mut timer, mut sprite, atlas_handle) in &mut query {
        if timer.tick(time.delta()).just_finished() {
            let atlas = textures.get(atlas_handle).unwrap();

            sprite.index = (sprite.index + 1) % atlas.textures.len();
        }
    }
}
