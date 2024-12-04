use bevy::prelude::*;

use crate::{despawn_screen, SystemState};

use super::{apply_menu_action, ButtonContainer, MenuButtonAction, MenuState, OnMainMenuScreen};

pub fn selection_option(app: &mut App) {
    app.add_systems(OnEnter(SystemState::Menu), setup_selection_option)
        .add_systems(OnExit(SystemState::Menu), despawn_screen::<SelectionButton>)
        .add_systems(
            Update,
            (
                button_system_mouse,
                button_system_keyboard,
                selected_option_system,
                animate_selected_option,
            )
                .chain()
                .run_if(in_state(SystemState::Menu)),
        );
}

#[derive(Component)]
struct SelectedOptionEgg {
    x_translation_anchor: f32,
}

#[derive(Component)]
pub struct SelectionButton(pub MenuButtonAction);

fn setup_selection_option(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            SelectionButton(MenuButtonAction::Play),
            Node {
                display: Display::Flex,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                // column_gap: Val::Px(40.),
                height: Val::Percent(100.),
                width: Val::Percent(100.),
                top: Val::Px(-180.0),
                left: Val::Px(300.0),
                ..Default::default()
            },
            ZIndex(i32::MAX),
            Visibility::Visible,
            BackgroundColor(Color::srgba(0.2, 0.0, 0.1, 0.3)),
        ))
        .with_children(|parent| {
            let egg = asset_server.load("egg.png");
            parent.spawn((
                SelectedOptionEgg {
                    x_translation_anchor: 250.,
                },
                ImageNode::new(egg.clone()),
                Node {
                    left: Val::Px(250.),
                    height: Val::Px(80.),
                    width: Val::Px(80.),
                    ..default()
                },
                Visibility::Visible,
                // ImageBundle {
                //     image: UiImage {
                //         texture: ),
                //         ..default()
                //     },
                // style: Style {
                //     left: Val::Px(250.),
                //     height: Val::Px(80.),
                //     width: Val::Px(80.),
                //     ..Default::default()
                // },
                // },
            ));
            parent.spawn((
                SelectedOptionEgg {
                    x_translation_anchor: -250.,
                },
                ImageNode::new(egg.clone()),
                Node {
                    left: Val::Px(-250.),
                    height: Val::Px(80.),
                    width: Val::Px(80.),
                    ..default()
                },
                Visibility::Visible,
            ));
        });
}

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);

// This system handles changing all buttons color based on mouse interaction
fn button_system_mouse(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &MenuButtonAction,
            Entity,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut selected_button: Query<&mut SelectionButton, Without<Button>>,
) {
    for (interaction, mut bg_color, mab, e) in &mut interaction_query {
        bg_color.0 = match *interaction {
            Interaction::None => NORMAL_BUTTON,
            Interaction::Hovered => {
                selected_button.single_mut().0 = mab.clone();

                HOVERED_BUTTON
            }
            _ => NORMAL_BUTTON,
        }
    }
}

fn button_system_keyboard(
    keys: Res<ButtonInput<KeyCode>>,
    curr_menu_state: Res<State<MenuState>>,
    app_exit_events: EventWriter<AppExit>,
    menu_state: ResMut<NextState<MenuState>>,
    game_state: ResMut<NextState<SystemState>>,
    mut selected_option: Query<&mut SelectionButton>,
    menu: Query<Entity, With<OnMainMenuScreen>>,
    commands: Commands,
) {
    if keys.just_pressed(KeyCode::Enter) || keys.just_pressed(KeyCode::Space) {
        apply_menu_action(
            curr_menu_state,
            &selected_option.single().0.clone(),
            app_exit_events,
            menu_state,
            game_state,
            selected_option,
            menu,
            commands,
        );
        return;
    }

    match curr_menu_state.get() {
        MenuState::Main | MenuState::IntroSequence => {
            let button_order = vec![
                MenuButtonAction::Play,
                MenuButtonAction::Settings,
                MenuButtonAction::About,
                MenuButtonAction::Quit,
            ];

            if let Some(curr_index) = button_order
                .iter()
                .position(|mba| *mba == selected_option.single().0)
            {
                if keys.just_pressed(KeyCode::ArrowUp) || keys.just_pressed(KeyCode::KeyW) {
                    selected_option.single_mut().0 = button_order
                        [(curr_index + button_order.len() - 1) % button_order.len()]
                    .clone();

                    // let thingo = buttons.iter().next().unwrap();
                    // // println!("left: {:?}", buttons.iter().next().unwrap().3.left);
                    // println!("top: {:?}", thingo.3.top);
                    // println!("left: {:?}", thingo.3.left);
                    // println!("translation: {:?}", thingo.0.translation);
                    // // println!("left: {:?}", thingo.3.left);
                }

                if keys.just_pressed(KeyCode::ArrowDown) || keys.just_pressed(KeyCode::KeyS) {
                    selected_option.single_mut().0 =
                        button_order[(curr_index + 1) % button_order.len()].clone();
                }
            }
        }
        _ => {}
    }
}

fn selected_option_system(
    mut buttons: Query<
        (&Transform, &mut BackgroundColor, &MenuButtonAction),
        (With<Button>, Without<ButtonContainer>),
    >,
    buttons_container: Query<&Transform, (Without<Button>, With<ButtonContainer>)>,
    mut selection_option: Query<
        (&mut Node, &SelectionButton),
        (Without<Button>, Without<ButtonContainer>),
    >,
) {
    buttons
        .iter_mut()
        .for_each(|(button_transform, mut bg_color, menu_action)| {
            // let (mut selection_transform, selection) = selected_option.single_mut();
            if let Ok((mut style, selection)) = selection_option.get_single_mut() {
                if *menu_action == selection.0 {
                    let button_container_translation_x = buttons_container
                        .get_single()
                        .map(|transform| transform.translation.x)
                        .ok();

                    match button_container_translation_x {
                        Some(x) => style.left = Val::Px(x + button_transform.translation.x),
                        None => style.left = Val::Px(button_transform.translation.x),
                    }
                    // if let Some(x) = button_container_translation_x {
                    //     style.left = Val::Px(x + button_transform.translation.x);
                    // }
                    style.top = Val::Px(button_transform.translation.y);

                    bg_color.0 = HOVERED_BUTTON;
                } else {
                    bg_color.0 = NORMAL_BUTTON;
                }
            }
        });
}

fn animate_selected_option(time: Res<Time>, mut query: Query<(&mut Node, &SelectedOptionEgg)>) {
    for (mut style, egg) in &mut query {
        style.left = Val::Px(
            egg.x_translation_anchor.signum() * 15.0 * f32::sin(time.elapsed_secs() * 4.)
                + egg.x_translation_anchor,
        );
    }
}
