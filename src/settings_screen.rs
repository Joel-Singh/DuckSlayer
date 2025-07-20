use crate::volume_settings::VolumeSettings;
use crate::widgets::checkbox::create_checkbox;
use crate::widgets::checkbox::Toggled;
use crate::widgets::slider::create_slider;
use crate::widgets::slider::Slid;
use bevy::audio::Volume;
use bevy::color::palettes::tailwind::*;
use bevy::prelude::*;

#[derive(Component)]
#[require(Name::new("Settings Screen"))]
struct SettingsScreen;

#[derive(Component)]
#[require(Name::new("Close Button"))]
struct CloseButton;

pub fn settings_screen_plugin(app: &mut App) {
    app.add_systems(Startup, spawn_settings_screen);
}

fn spawn_settings_screen(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    volume: Res<VolumeSettings>,
) {
    let settings_screen = commands
        .spawn((
            SettingsScreen,
            Node {
                display: Display::None,
                position_type: PositionType::Absolute,
                width: Val::Vw(50.),
                height: Val::Vh(50.),
                margin: UiRect::AUTO,
                border: UiRect::all(Val::Px(30.)),

                align_items: AlignItems::Center,
                justify_items: JustifyItems::Center,

                grid_template_rows: RepeatedGridTrack::auto(3),
                grid_template_columns: RepeatedGridTrack::auto(3),

                ..default()
            },
            BackgroundColor(CYAN_500.into()),
            BorderRadius::all(Val::Percent(10.)),
            BorderColor(CYAN_800.into()),
            ZIndex(1),
        ))
        .with_children(|p| {
            // Bar holding the X, had to do some hackery to get the X in the top right
            p.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    width: Val::Percent(100.),
                    flex_direction: FlexDirection::RowReverse,
                    align_self: AlignSelf::Start,
                    ..default()
                },
                Name::new("Close Button Container"),
            ))
            .with_children(|p| {
                p.spawn((
                    CloseButton,
                    ImageNode::new(asset_server.load("cross-icon.png")),
                    Node {
                        margin: UiRect::all(Val::Px(5.)),
                        width: Val::Px(30.),
                        height: Val::Px(33.),
                        ..default()
                    },
                ))
                .observe(|_: Trigger<Pointer<Click>>, mut commands: Commands| {
                    commands.queue(HideSettingsScreen);
                });
            });
        })
        .id();

    let mute_sfx_checkbox = create_checkbox(&mut commands, volume.get_sfx_mute());
    commands
        .entity(mute_sfx_checkbox)
        .observe(
            |toggled: Trigger<Toggled>, mut volume: ResMut<VolumeSettings>| {
                volume.set_sfx_mute(toggled.is_checked);
            },
        )
        .entry::<Node>()
        .and_modify(|mut node| {
            node.grid_row = GridPlacement::start(2);
            node.grid_column = GridPlacement::start(2);
        });

    let mute_music_checkbox = create_checkbox(&mut commands, volume.get_music_mute());
    commands
        .entity(mute_music_checkbox)
        .observe(
            |toggled: Trigger<Toggled>, mut volume: ResMut<VolumeSettings>| {
                volume.set_music_mute(toggled.is_checked);
            },
        )
        .entry::<Node>()
        .and_modify(|mut node| {
            node.grid_row = GridPlacement::start(3);
            node.grid_column = GridPlacement::start(2);
        });

    let sfx_slider = create_slider(&mut commands, volume.get_sfx().to_linear());
    commands
        .entity(sfx_slider)
        .observe(
            |trigger: Trigger<Slid>, mut volume: ResMut<VolumeSettings>| {
                volume.set_sfx_vol(Volume::Linear(trigger.slid_percentage));
            },
        )
        .entry::<Node>()
        .and_modify(|mut node| {
            node.grid_row = GridPlacement::start(2);
            node.grid_column = GridPlacement::start(3);
        });

    let music_slider = create_slider(&mut commands, volume.get_music().to_linear());
    commands
        .entity(music_slider)
        .observe(
            |trigger: Trigger<Slid>, mut volume: ResMut<VolumeSettings>| {
                volume.set_music_vol(Volume::Linear(trigger.slid_percentage));
            },
        )
        .entry::<Node>()
        .and_modify(|mut node| {
            node.grid_row = GridPlacement::start(3);
            node.grid_column = GridPlacement::start(3);
        });

    commands
        .entity(settings_screen)
        .add_child(mute_sfx_checkbox)
        .add_child(mute_music_checkbox)
        .add_child(sfx_slider)
        .add_child(music_slider)
        .with_children(|p| {
            let create_text = |text: &'static str, grid_row: i16, grid_column: i16| {
                (
                    Text::new(text),
                    TextFont {
                        font: asset_server.load("DynaPuff-Regular.ttf"),
                        ..default()
                    },
                    TextColor(Color::BLACK),
                    Node {
                        grid_row: GridPlacement::start(grid_row),
                        grid_column: GridPlacement::start(grid_column),
                        ..default()
                    },
                )
            };

            p.spawn(create_text("Mute", 1, 2));
            p.spawn(create_text("Volume", 1, 3));
            p.spawn(create_text("SFX", 2, 1));
            p.spawn(create_text("Music", 3, 1));
        });
}

pub struct ShowSettingsScreen;
impl Command for ShowSettingsScreen {
    fn apply(self, mut world: &mut World) {
        let settings_screen: Entity = world
            .query_filtered::<Entity, With<SettingsScreen>>()
            .single(&mut world)
            .unwrap();

        let mut node = world.get_mut::<Node>(settings_screen).unwrap();
        node.display = Display::Grid;
    }
}

struct HideSettingsScreen;
impl Command for HideSettingsScreen {
    fn apply(self, mut world: &mut World) {
        let settings_screen: Entity = world
            .query_filtered::<Entity, With<SettingsScreen>>()
            .single(&mut world)
            .unwrap();

        let mut node = world.get_mut::<Node>(settings_screen).unwrap();
        node.display = Display::None;
    }
}
