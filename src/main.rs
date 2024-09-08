use bevy::{
    prelude::{App, ClearColor, Color, DefaultPlugins, States},
    winit::WinitSettings,
};

mod animate_tiles;
mod area_multiplier;
mod combo_multiplier;
mod conversions;
mod menu;
// mod sound_effects;
mod squarea_core;
mod timer;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // .insert_resource(WinitSettings::desktop_app())
        .add_plugins(squarea_core::SquareaCore)
        .add_plugins(combo_multiplier::ComboMultiplier)
        .add_plugins(area_multiplier::AreaMultiplier)
        .add_plugins(animate_tiles::AnimateTiles)
        .add_plugins(timer::SquareaTimer)
        .add_plugins(menu::SquareaMenu)
        .insert_resource(ClearColor(Color::srgb(0.99, 0.5, 0.5)))
        .run();
}
