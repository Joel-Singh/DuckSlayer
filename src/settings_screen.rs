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
    commands
        .spawn((
            SettingsScreen,
            Node {
                display: Display::None,
                position_type: PositionType::Absolute,
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
            p.spawn((
                CloseButton,
                ImageNode::new(asset_server.load("cross-icon.png")),
                Node {
                    margin: UiRect::all(Val::Px(15.)).with_left(Val::Auto),
                    // From the file
                    width: Val::Px(30.),
                    height: Val::Px(33.),
                    ..default()
                },
            ))
            .observe(|_: Trigger<Pointer<Click>>, mut commands: Commands| {
                commands.queue(HideSettingsScreen);
            });
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
