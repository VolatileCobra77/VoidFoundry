use bevy::prelude::*;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (startup))
        .add_systems(Update, (update))
        .run();
}

fn startup(mut commands: Commands) {

}

fn update(){

}
