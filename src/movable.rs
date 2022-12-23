use bevy::prelude::*;

#[derive(Component)]
pub struct Movable;

pub struct MovePlugin;

const SPEED: f32 = 80.0;

impl Plugin for MovePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(move_entity);
    }
}

fn move_entity(
    mut items_query: Query<&mut Transform, With<Movable>>,
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    for mut item in &mut items_query {
        if keys.pressed(KeyCode::Up) {
            let direction = item.local_y();
            item.translation += direction * (SPEED * time.delta_seconds());
        }
        if keys.pressed(KeyCode::Down) {
            let direction = item.local_y();
            item.translation -= direction * (SPEED * time.delta_seconds());
        }
        if keys.pressed(KeyCode::Left) {
            let direction = item.local_x();
            item.translation -= direction * (SPEED * time.delta_seconds());
        }
        if keys.pressed(KeyCode::Right) {
            let direction = item.local_x();
            item.translation += direction * (SPEED * time.delta_seconds());
        }
    }
}
