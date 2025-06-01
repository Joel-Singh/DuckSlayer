use bevy::{input::common_conditions::input_just_pressed, prelude::*};
use game_messages::set_message;
use DuckSlayer::delete_all;

use crate::{
    deckbar::{clear_deckbar, push_to_deckbar, show_deckbar, Card, DeckBarRoot, PushToDeckbar},
    global::{GameState, NEST_FIRST_X, NEST_SECOND_X, NEST_Y, QUAKKA_STARTING_POSITION},
    troops::{spawn_nest, troop_bundles::spawn_troop, Bridge, Farmer, Nest, Quakka},
};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, States)]
pub enum IsPaused {
    True,
    False,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, States, Default)]
pub enum GameOver {
    True,
    #[default]
    False,
}

#[derive(Resource)]
struct LevelRes {
    troops: Vec<(Card, Vec2)>,
    nest_locations: Vec<Vec2>,
    starting_deckbar: Vec<Card>,
}

impl Default for LevelRes {
    fn default() -> Self {
        return LevelRes {
            troops: vec![(Card::Quakka, QUAKKA_STARTING_POSITION)],
            nest_locations: vec![
                (NEST_FIRST_X, NEST_Y).into(),
                (NEST_SECOND_X, NEST_Y).into(),
            ],
            starting_deckbar: vec![Card::Farmer, Card::Waterball],
        };
    }
}

impl LevelRes {
    fn clear(&mut self) {
        *self = LevelRes {
            troops: Vec::new(),
            nest_locations: Vec::new(),
            starting_deckbar: Vec::new(),
        };
    }
}

#[derive(Component, Default)]
pub struct LevelEntity;

pub fn manage_level(app: &mut App) {
    app.add_plugins(game_messages::game_messages)
        .add_plugins(debug_ui::debug_ui_plugin)
        .add_systems(
            OnEnter(GameState::InGame),
            (
                spawn_entities,
                spawn_arena_background,
                show_deckbar,
                push_to_deckbar(Card::Farmer),
                set_message("[Space] to start level"),
            ),
        )
        .add_systems(
            FixedUpdate,
            (
                ((
                    delete_all::<LevelEntity>,
                    clear_deckbar,
                    spawn_entities_from_level,
                    pause,
                    set_gameover_false,
                    set_message("[Space] to start level"),
                )
                    .chain()
                    .run_if(input_just_pressed(KeyCode::KeyZ))),
                unpause.run_if(input_just_pressed(KeyCode::Space).and(in_state(GameOver::False))),
                set_gameover_true.run_if(nest_destroyed),
            )
                .run_if(in_state(GameState::InGame)),
        )
        .add_systems(OnEnter(IsPaused::False), set_message(""))
        .add_systems(
            OnEnter(GameOver::True),
            (pause, set_message("Gameover: nest destroyed")),
        )
        .insert_state::<IsPaused>(IsPaused::True)
        .init_state::<GameOver>()
        .init_resource::<LevelRes>();
}

fn spawn_arena_background(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Sprite {
            image: asset_server.load("arena_background.png"),
            ..default()
        },
        Transform {
            // -0.5 so it's in the back and clicks are registered to Nodes
            translation: Vec3::new(0., 0., -0.5),
            ..default()
        },
    ));
}

fn spawn_entities(asset_server: Res<AssetServer>, mut commands: Commands) {
    spawn_troop(
        Card::Quakka,
        QUAKKA_STARTING_POSITION,
        &mut commands,
        &asset_server,
    );

    spawn_nest((NEST_FIRST_X, NEST_Y).into(), &mut commands, &asset_server);

    spawn_nest((NEST_SECOND_X, NEST_Y).into(), &mut commands, &asset_server);

    commands.spawn((
        Bridge,
        Transform {
            translation: Vec3::new(-392.0, -4.0, 0.),
            ..default()
        },
    ));

    commands.spawn((
        Bridge,
        Transform {
            translation: Vec3::new(165.0, -8.0, 0.),
            ..default()
        },
    ));
}

fn save_level(
    mut level: ResMut<LevelRes>,
    level_entities: Query<(&Transform, Has<Quakka>, Has<Farmer>, Has<Nest>), With<LevelEntity>>,
    cards: Query<&Card>,
    deck: Single<&Children, With<DeckBarRoot>>,
) {
    level.clear();

    for (transform, is_quakka, is_farmer, is_nest) in level_entities {
        if is_quakka {
            level
                .troops
                .push((Card::Quakka, transform.translation.truncate()));
        } else if is_farmer {
            level
                .troops
                .push((Card::Farmer, transform.translation.truncate()));
        } else if is_nest {
            level.nest_locations.push(transform.translation.truncate());
        }
    }

    for card_e in deck.into_inner() {
        level.starting_deckbar.push(*cards.get(*card_e).unwrap());
    }
}

fn spawn_entities_from_level(
    level: Res<LevelRes>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    for (card, position) in &level.troops {
        spawn_troop(*card, *position, &mut commands, &asset_server);
    }

    for nest_position in &level.nest_locations {
        spawn_nest(*nest_position, &mut commands, &asset_server);
    }

    for card in &level.starting_deckbar {
        commands.queue(PushToDeckbar(*card));
    }
}

fn unpause(mut is_paused: ResMut<NextState<IsPaused>>) {
    is_paused.set(IsPaused::False);
}

fn pause(mut is_paused: ResMut<NextState<IsPaused>>) {
    is_paused.set(IsPaused::True);
}

fn set_gameover_true(mut gameover: ResMut<NextState<GameOver>>) {
    gameover.set(GameOver::True);
}

fn set_gameover_false(mut gameover: ResMut<NextState<GameOver>>) {
    gameover.set(GameOver::False);
}

fn nest_destroyed(nests: Query<(), With<Nest>>) -> bool {
    nests.iter().count() < 2
}

mod debug_ui {
    use bevy::{ecs::system::RunSystemOnce, prelude::*};
    use bevy_egui::{
        egui::{self, Ui},
        EguiContextPass, EguiContexts, EguiPlugin,
    };
    use strum::IntoEnumIterator;

    use crate::{
        deckbar::{Card, PushToDeckbar},
        global::in_debug,
    };

    use super::save_level;

    pub fn debug_ui_plugin(app: &mut App) {
        app.add_plugins(EguiPlugin {
            enable_multipass_for_primary_context: true,
        })
        .add_systems(EguiContextPass, create_debug_window.run_if(in_debug));
    }

    fn create_debug_window(mut contexts: EguiContexts, mut commands: Commands) {
        egui::Window::new("DEBUG UI").show(contexts.ctx_mut(), |ui| {
            create_push_to_deckbar_btns(ui, &mut commands);
            if ui.button("Save level").clicked() {
                commands.queue(move |world: &mut World| {
                    let _ = world.run_system_once(save_level);
                })
            }
        });
    }

    fn create_push_to_deckbar_btns(ui: &mut Ui, commands: &mut Commands) {
        for card in Card::iter() {
            if card.is_empty() {
                continue;
            }

            let push_to_deck_btn = ui.button("Push ".to_string() + &card.to_string());
            if push_to_deck_btn.clicked() {
                commands.queue(PushToDeckbar(card));
            }
        }
    }
}

mod game_messages {
    use bevy::{
        ecs::{schedule::ScheduleConfigs, system::ScheduleSystem},
        prelude::*,
    };

    #[derive(Component)]
    struct MessageBox;

    pub fn game_messages(app: &mut App) {
        app.add_systems(Startup, spawn_message_box);
    }

    fn spawn_message_box(mut commands: Commands) {
        commands.spawn((
            Text::new(""),
            Node {
                margin: UiRect::all(Val::Auto).with_top(Val::Px(80.)),
                ..default()
            },
            BoxShadow::new(
                Color::BLACK.with_alpha(0.8),
                Val::ZERO,
                Val::ZERO,
                Val::Percent(0.),
                Val::Px(10.),
            ),
            MessageBox,
        ));
    }

    pub fn set_message(message: &'static str) -> ScheduleConfigs<ScheduleSystem> {
        return (move |mut commands: Commands, message_box: Single<Entity, With<MessageBox>>| {
            commands.entity(*message_box).insert(Text::new(message));
        })
        .into_configs();
    }
}
