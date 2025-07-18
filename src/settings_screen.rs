use crate::volume_settings::VolumeSettings;
use crate::widgets::checkbox::create_checkbox;
use crate::widgets::checkbox::Toggled;
use crate::widgets::slider::create_slider;
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

fn spawn_settings_screen(mut commands: Commands, asset_server: Res<AssetServer>) {
    let settings_screen = commands
        .spawn((
            SettingsScreen,
            Node {
                display: Display::None,
                position_type: PositionType::Absolute,
                flex_direction: FlexDirection::Column,
                width: Val::Vw(50.),
                height: Val::Vh(50.),
                margin: UiRect::AUTO,
                border: UiRect::all(Val::Px(30.)),
                ..default()
            },
            BackgroundColor(CYAN_500.into()),
            BorderRadius::all(Val::Percent(10.)),
            BorderColor(CYAN_800.into()),
            ZIndex(1),
        ))
        .with_children(|p| {
            // Bar holding the X, had to do some hackery to get the X in the top right
            p.spawn(Node {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.),
                flex_direction: FlexDirection::RowReverse,
                ..default()
            })
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

    let mute_sfx_checkbox = create_checkbox(&mut commands);
    commands.entity(mute_sfx_checkbox).observe(
        |toggled: Trigger<Toggled>, mut volume: ResMut<VolumeSettings>| {
            volume.set_sfx_mute(toggled.is_checked);
        },
    );

    let mute_music_checkbox = create_checkbox(&mut commands);
    commands.entity(mute_music_checkbox).observe(
        |toggled: Trigger<Toggled>, mut volume: ResMut<VolumeSettings>| {
            volume.set_music_mute(toggled.is_checked);
        },
    );

    let slider = create_slider(&mut commands);

    commands
        .entity(settings_screen)
        .add_child(mute_sfx_checkbox)
        .add_child(mute_music_checkbox)
        .add_child(slider);
}

pub struct ShowSettingsScreen;
impl Command for ShowSettingsScreen {
    fn apply(self, mut world: &mut World) {
        let settings_screen: Entity = world
            .query_filtered::<Entity, With<SettingsScreen>>()
            .single(&mut world)
            .unwrap();

        let mut node = world.get_mut::<Node>(settings_screen).unwrap();
        node.display = Display::Flex;
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
