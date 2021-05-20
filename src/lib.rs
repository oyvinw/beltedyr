use bevy::prelude::*;
use bevy::render::pass::ClearColor;
use ldtk_rust::*;

mod helper;

//Path this to LDtk-file if needed
const LDTK_FILE_PATH: &str = "assets/ldtk/ldtk_project.ldtk";


pub fn run (){
    let project = Project::new(LDTK_FILE_PATH.to_string());

    App::build()
    .add_plugins(DefaultPlugins)
    .insert_resource(WindowDescriptor {
        title: "beltedyr".to_string(),
        ..Default::default()
    })
    .insert_resource(ClearColor(helper::hex(&project.bg_color).unwrap()))
    .add_system(setup.system())
    .run();
}

fn setup(mut commands: Commands) {
    //Do init setup
    commands
        .spawn()
        .insert_bundle(OrthographicCameraBundle::new_2d());
}