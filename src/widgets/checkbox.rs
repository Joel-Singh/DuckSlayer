use bevy::prelude::*;

#[derive(Component)]
#[require(Name::new("Checkbox"))]
struct Checkbox {
    is_checked: bool,
}

#[derive(Event)]
pub struct Toggled {
    pub is_checked: bool,
}

pub fn checkbox_plugin(_app: &mut App) {}

pub fn create_checkbox(commands: &mut Commands) -> Entity {
    commands
        .spawn((
            Checkbox { is_checked: false },
            Node {
                width: Val::Px(50.),
                height: Val::Px(50.),
                border: UiRect::all(Val::Px(5.)),
                ..default()
            },
            BorderColor(Color::BLACK),
            BackgroundColor(Srgba::hex("#ffd966ff").unwrap().into()),
        ))
        .with_children(|p| {
            p.spawn((
                Node {
                    display: Display::None,
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    border: UiRect::all(Val::Px(5.)),
                    ..default()
                },
                BorderColor(Color::BLACK),
                BackgroundColor(Srgba::hex("#ffd966ff").unwrap().into()),
                BorderRadius::MAX,
                Pickable {
                    should_block_lower: false,
                    is_hoverable: false,
                },
            ));
        })
        .observe(
            |trigger: Trigger<Pointer<Click>>,
             mut checkbox_q: Query<&mut Checkbox>,
             children_q: Query<&Children>,
             mut commands: Commands| {
                let mut checkbox = checkbox_q.get_mut(trigger.target).unwrap();
                checkbox.is_checked = !checkbox.is_checked;

                let checkbox_checked = checkbox.is_checked; // Copy because checkbox doesn't live long enough
                for child in children_q.get(trigger.target).unwrap() {
                    commands
                        .entity(*child)
                        .entry::<Node>()
                        .and_modify(move |mut node| {
                            node.display = if checkbox_checked {
                                Display::DEFAULT
                            } else {
                                Display::None
                            }
                        });
                }

                commands.entity(trigger.target).trigger(Toggled {
                    is_checked: checkbox.is_checked,
                });
            },
        )
        .id()
}
