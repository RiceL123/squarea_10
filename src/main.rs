use bevy::prelude::{App, ClearColor, Color, DefaultPlugins};

mod area_multiplier;
mod combo_multiplier;
mod squarea_core;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(squarea_core::SquareaCore)
        .add_plugins(combo_multiplier::ComboMultiplier)
        .add_plugins(area_multiplier::AreaMultiplier)
        .insert_resource(ClearColor(Color::srgb(1., 0.3, 0.4)))
        .run();
}
