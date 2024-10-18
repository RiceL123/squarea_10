use bevy::{audio::Volume, input::common_conditions::input_just_pressed, prelude::*};

pub struct Sound;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
struct MyAudioSet;

#[derive(Component)]
struct BackgroundTrack;

#[derive(Resource, Debug)]
pub struct Audio {
    is_enabled: bool,
    volume: f32,
}

#[derive(Component)]
struct VolumeBarContainer;

#[derive(Component)]
struct VolumeBar;

#[derive(Component)]
struct HitSound;

#[derive(Event)]
pub struct HitEvent(pub i32);

#[derive(Event)]
struct ToggleBgmEvent;

#[derive(Component)]
struct ToggleBgmButton;

#[derive(Component)]
struct SoundIcon;

impl Plugin for Sound {
    fn build(&self, app: &mut App) {
        app.insert_resource(Audio {
            is_enabled: true,
            volume: 1.0,
        });
        app.add_event::<ToggleBgmEvent>();
        app.add_event::<HitEvent>();
        app.add_systems(
            Startup,
            (play_music.run_if(is_audio_enabled),).in_set(MyAudioSet),
        );
        app.add_systems(
            Update,
            (
                toggle_audio.run_if(input_just_pressed(KeyCode::KeyM)),
                // play_sfx.run_if(input_just_pressed(KeyCode::KeyP)),
                play_sfx,
                change_volume,
                button_system,
                toggle_bgm,
            )
                .in_set(MyAudioSet),
        );
        app.observe(write_sfx_event);

        app.configure_sets(Update, MyAudioSet);

        app.add_systems(Startup, init_sound);
    }
}

fn init_sound(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(NodeBundle {
            style: Style {
                display: Display::Flex,
                justify_self: JustifySelf::End,
                align_self: AlignSelf::End,
                align_items: AlignItems::Center,
                column_gap: Val::Px(10.),
                margin: UiRect::all(Val::Px(20.)),
                ..default()
            },
            z_index: ZIndex::Global(100),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "toggle bgm",
                TextStyle {
                    font_size: 30.0,
                    color: Color::srgb(0.9, 0.9, 0.9),
                    ..default()
                },
            ));

            parent
                .spawn((
                    ToggleBgmButton,
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(60.0),
                            height: Val::Px(60.0),
                            border: UiRect::all(Val::Px(5.0)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            padding: UiRect::all(Val::Px(5.)),
                            ..default()
                        },
                        border_color: BorderColor(Color::BLACK),
                        border_radius: BorderRadius::all(Val::Px(10.)),
                        background_color: Color::srgb(0.9, 0.9, 0.9).into(),
                        ..default()
                    },
                ))
                .with_children(|parent| {
                    parent.spawn((
                        SoundIcon,
                        NodeBundle {
                            style: Style {
                                width: Val::Percent(100.0),
                                height: Val::Percent(100.0),
                                ..default()
                            },
                            ..default()
                        },
                        UiImage::new(asset_server.load("muted-icon.png")),
                    ));
                });

            parent
                .spawn((
                    VolumeBarContainer,
                    NodeBundle {
                        style: Style {
                            width: Val::Px(200.),
                            height: Val::Px(20.),
                            ..default()
                        },
                        background_color: Color::srgba(0.3, 0.3, 0.3, 0.8).into(),
                        // z_index: ZIndex::Global(10),
                        border_radius: BorderRadius::all(Val::Px(10.)),
                        ..default()
                    },
                ))
                .with_children(|parent| {
                    parent.spawn((
                        VolumeBar,
                        NodeBundle {
                            style: Style {
                                width: Val::Percent(1. / 7. * 100.),
                                height: Val::Percent(100.),
                                border: UiRect::all(Val::Px(2.)),
                                ..default()
                            },
                            background_color: Color::srgba(1., 1., 1., 0.5).into(),
                            border_radius: BorderRadius::all(Val::Px(10.)),
                            ..default()
                        },
                    ));
                });
        });
}

fn is_audio_enabled(audio: Res<Audio>) -> bool {
    audio.is_enabled
}

fn toggle_audio(mut ev_writer: EventWriter<ToggleBgmEvent>) {
    ev_writer.send(ToggleBgmEvent);
}

fn toggle_bgm(
    mut ev_reader: EventReader<ToggleBgmEvent>,
    mut audio_res: ResMut<Audio>,
    mut audio_bundle: Query<&mut AudioSink, With<BackgroundTrack>>,
    mut volume_bar: Query<&mut BackgroundColor, With<VolumeBar>>,
    asset_server: Res<AssetServer>,
    mut button_icon: Query<&mut UiImage, With<SoundIcon>>,
) {
    for _ in ev_reader.read() {
        audio_res.as_mut().is_enabled = match audio_res.is_enabled {
            true => {
                volume_bar.single_mut().0 = Color::srgba(0.9, 0.1, 0.1, 0.5).into();
                audio_bundle.single_mut().pause();
                button_icon.single_mut().texture = asset_server.load("sound-icon.png");
                false
            }
            false => {
                volume_bar.single_mut().0 = Color::srgba(1., 1., 1., 0.5).into();
                audio_bundle.single_mut().play();
                button_icon.single_mut().texture = asset_server.load("muted-icon.png");
                true
            }
        };
    }
}

use bevy::input::mouse::MouseWheel;

use crate::conversions::IntBounds;
use crate::squarea_core::PopTiles;

fn change_volume(
    mut evr_scroll: EventReader<MouseWheel>,
    mut ev_click: EventWriter<HitEvent>,
    music_controller: Query<&AudioSink, With<BackgroundTrack>>,
    sfx_controller: Query<&AudioSink, With<HitSound>>,
    mut audio_res: ResMut<Audio>,
    mut volume_set: ParamSet<(
        Query<&mut Style, With<VolumeBar>>,
        Query<&mut Style, With<VolumeBarContainer>>,
    )>,
) {
    use bevy::input::mouse::MouseScrollUnit;

    if let Ok(sink) = music_controller.get_single() {
        for ev in evr_scroll.read() {
            match ev.unit {
                MouseScrollUnit::Line => {
                    audio_res.volume = (audio_res.volume + (ev.y / 5.0)).clamp(0., 7.);

                    sink.set_volume(audio_res.volume);
                    volume_set.p0().single_mut().width = Val::Percent(audio_res.volume / 7. * 100.);

                    if let Ok(hitsound) = sfx_controller.get_single() {
                        hitsound.set_volume(audio_res.volume);
                    };
                    ev_click.send(HitEvent(1));
                }
                MouseScrollUnit::Pixel => {
                    audio_res.volume = (audio_res.volume + (ev.y / 10.0)).clamp(0., 7.);

                    sink.set_volume(audio_res.volume);
                    volume_set.p0().single_mut().width = Val::Percent(audio_res.volume / 7. * 100.);

                    if let Ok(hitsound) = sfx_controller.get_single() {
                        hitsound.set_volume(audio_res.volume);
                    };
                    ev_click.send(HitEvent(1));
                }
            }
        }
    }
}

fn play_music(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        BackgroundTrack,
        AudioBundle {
            source: asset_server.load("bg_track.ogg"),
            settings: PlaybackSettings::LOOP,
            ..default()
        },
    ));
}

fn play_sfx(
    mut ev_reader: EventReader<HitEvent>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
) {
    for x in ev_reader.read() {
        // println!("{:?}", x.0);
        commands.spawn((
            HitSound,
            AudioBundle {
                source: asset_server.load("hit_sound.ogg"),
                settings: PlaybackSettings::DESPAWN.with_volume(Volume::new(audio.volume)),
                ..default()
            },
        ));
    }
}

fn button_system(
    mut ev_writer: EventWriter<ToggleBgmEvent>,
    mut ev_click: EventWriter<HitEvent>,

    mut interaction_query: Query<
        (&Interaction, &mut BorderColor),
        (Changed<Interaction>, With<ToggleBgmButton>),
    >,
) {
    if let Ok(query) = &mut interaction_query.get_single_mut() {
        match *query.0 {
            Interaction::Pressed => {
                ev_writer.send(ToggleBgmEvent);
                ev_click.send(HitEvent(1));
            }
            Interaction::Hovered => {
                query.1 .0 = Color::srgb(0.1, 0.2, 0.7);
            }
            _ => {
                query.1 .0 = Color::srgb(0.9, 0.9, 0.9);
            }
        }
    }
}

fn write_sfx_event(
    trigger: Trigger<PopTiles>,
    mut commands: Commands,
    mut ev_writer: EventWriter<HitEvent>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
) {
    // ev_writer.send(HitEvent(trigger.event().0.iter().count() as i32));

    let bounds = IntBounds::from_positions(trigger.event().0.iter().map(|(_, p)| p).collect());

    let width = bounds.right - bounds.left + 1;
    let height = bounds.upper - bounds.lower + 1;
    let area = height * width;

    match area {
        0..3 => {
            commands.spawn(AudioBundle {
                source: asset_server.load("sounds/area1.ogg"),
                settings: PlaybackSettings::DESPAWN.with_volume(Volume::new(audio.volume)),
                ..default()
            });
        }
        3..6 => {
            commands.spawn(AudioBundle {
                source: asset_server.load("sounds/area2.ogg"),
                settings: PlaybackSettings::DESPAWN.with_volume(Volume::new(audio.volume)),
                ..default()
            });
        }
        6..9 => {
            commands.spawn(AudioBundle {
                source: asset_server.load("sounds/area3.ogg"),
                settings: PlaybackSettings::DESPAWN.with_volume(Volume::new(audio.volume)),
                ..default()
            });
        }
        9..16 => {
            commands.spawn(AudioBundle {
                source: asset_server.load("sounds/area4.ogg"),
                settings: PlaybackSettings::DESPAWN.with_volume(Volume::new(audio.volume)),
                ..default()
            });
        }
        _ => {
            commands.spawn(AudioBundle {
                source: asset_server.load("sounds/area5.ogg"),
                settings: PlaybackSettings::DESPAWN.with_volume(Volume::new(audio.volume)),
                ..default()
            });
        }
    }
}
