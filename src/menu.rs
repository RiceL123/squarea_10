use bevy::prelude::*;

pub struct SquareaMenu;

impl Plugin for SquareaMenu {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_menu);
    }
}

fn init_menu(mut commands: Commands) {
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Px(500.),
                height: Val::Px(700.),
                justify_self: JustifySelf::Center,
                align_self: AlignSelf::Center,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceEvenly,
                flex_direction: FlexDirection::Row,
                ..default()
            },
            background_color: Color::srgb(0.3, 0.7, 0.3).into(),
            z_index: ZIndex::Global(10),
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Px(200.),
                        height: Val::Px(200.),
                        border: UiRect::all(Val::Px(2.)),
                        ..default()
                    },
                    background_color: Color::srgb(0.65, 0.65, 0.65).into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(NodeBundle {
                        style: Style {
                            width: Val::Percent(80.),
                            flex_direction: FlexDirection::Row,
                            padding: UiRect::all(Val::Px(5.)),
                            row_gap: Val::Px(5.),
                            ..default()
                        },
                        background_color: Color::srgb(0.15, 0.15, 0.15).into(),
                        ..default()
                    });
                });

            parent.spawn(NodeBundle {
                style: Style {
                    width: Val::Px(200.),
                    height: Val::Px(200.),
                    border: UiRect::all(Val::Px(2.)),
                    ..default()
                },
                background_color: Color::srgb(0.9, 0.65, 0.65).into(),
                ..default()
            });

            parent.spawn(NodeBundle {
                style: Style {
                    width: Val::Px(200.),
                    height: Val::Px(200.),
                    border: UiRect::all(Val::Px(2.)),
                    ..default()
                },
                background_color: Color::srgb(0.9, 0.65, 0.8).into(),
                ..default()
            });
        });
}
