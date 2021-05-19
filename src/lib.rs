use bevy::prelude::*;
use bevy::render::pass::ClearColor;

mod ldtk;

pub fn run (){
    App::build()
    .add_plugins(DefaultPlugins)
    .insert_resource(WindowDescriptor {
        title: "beltedyr".to_string(),
        ..Default::default()
    })
    .insert_resource(ClearColor(Color::rgb(0.7, 0.7, 0.7)))
    .add_system(setup.system())
    .run();
}

fn setup(mut commands: Commands) {
    //Do init setup
}