use bevy::prelude::*;

pub struct GreetPlugin;

#[derive(Resource)]
struct GreetTimer(Timer);

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);


impl Plugin for GreetPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(hello_world)
            .add_startup_system(add_people)
            .insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
            .add_system(greet_people);
    }
}

fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Mrunmay".to_owned())));
    commands.spawn((Person, Name("Jack".to_owned())));
    commands.spawn((Person, Name("Nina".to_owned())));
}

fn greet_people(people: Query<&Name, With<Person>>, mut timer: ResMut<GreetTimer>, time: Res<Time>) {
    if timer.0.tick(time.delta()).just_finished() {
        for name in people.iter() {
            println!("Hello {}",name.0);
        }
    }
}

fn hello_world(_commands: Commands) {
    println!("Hello game world")
}
