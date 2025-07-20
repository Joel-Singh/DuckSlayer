use bevy::prelude::*;

const SLIDER_WIDTH: f32 = 200.;
const SLIDER_HEIGHT: f32 = 50.;
const SLIDER_BORDER: f32 = 10.;
const DRAG_START_PLACEHOLDER: f32 = 0.; // Not actually important because it gets immediately overwritten
const HIGHEST_LEFT: f32 = SLIDER_WIDTH - SLIDER_HEIGHT;

#[derive(Component)]
#[require(Name::new("Slider"))]
struct Slider {
    drag_start: f32, // in percentage, from 0 to 1
    stud: Entity,
}

#[derive(Event)]
pub struct Slid {
    pub slid_percentage: f32, // From 0.0 to 1.0
}

// starting_slid_percentage should be from 0.0 to 1.0
pub fn create_slider(commands: &mut Commands, starting_slid_percentage: f32) -> Entity {
    let slider_node = Node {
        align_items: AlignItems::Center,
        width: Val::Px(SLIDER_WIDTH),
        height: Val::Px(SLIDER_HEIGHT),
        border: UiRect::all(Val::Px(SLIDER_BORDER)),
        ..default()
    };

    let slider = commands
        .spawn((
            slider_node.clone(),
            BorderColor(Color::BLACK),
            BackgroundColor(Srgba::hex("#ffd966ff").unwrap().into()),
        ))
        .id();

    let stud = commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                width: Val::Px(SLIDER_HEIGHT),
                height: Val::Px(SLIDER_HEIGHT),
                border: UiRect::all(Val::Px(SLIDER_BORDER)),
                left: Val::Px(starting_slid_percentage * HIGHEST_LEFT - SLIDER_BORDER),
                ..default()
            },
            BorderColor(Color::BLACK),
            BackgroundColor(Srgba::hex("#ffd966ff").unwrap().into()),
            Pickable {
                should_block_lower: false,
                is_hoverable: false,
            },
            BorderRadius::MAX,
        ))
        .id();

    commands
        .entity(slider)
        .insert(Slider {
            stud,
            drag_start: DRAG_START_PLACEHOLDER,
        })
        .add_child(stud)
        .observe(
            |trigger: Trigger<Pointer<Pressed>>,
             node_q: Query<&mut Node>,
             slider_q: Query<&Slider>,
             mut commands: Commands| {
                // trigger.hit.position.unwrap().x is from 0 to 1
                let slider = slider_q.get(trigger.target).unwrap();

                slider_callback(
                    trigger.hit.position.unwrap().x,
                    &slider,
                    trigger.target,
                    node_q,
                    &mut commands,
                );
            },
        )
        .observe(
            |trigger: Trigger<Pointer<DragStart>>, mut slider_q: Query<&mut Slider>| {
                let mut slider = slider_q.get_mut(trigger.target).unwrap();
                slider.drag_start = trigger.hit.position.unwrap().x;
            },
        )
        .observe(
            |trigger: Trigger<Pointer<Drag>>,
             node_q: Query<&mut Node>,
             slider_q: Query<&Slider>,
             mut commands: Commands| {
                let slider = slider_q.get(trigger.target).unwrap();
                // trigger.distance.x is in pixels, not percentage like slider.drag_start
                let mut percentage: f32 = slider.drag_start + (trigger.distance.x / SLIDER_WIDTH); // From 0 to 1
                percentage = percentage.min(1.).max(0.);

                slider_callback(percentage, &slider, trigger.target, node_q, &mut commands);
            },
        );

    return slider;
}

fn slider_callback(
    percentage: f32, // from 0 to 1
    slider: &Slider,
    slider_e: Entity,
    mut node_q: Query<&mut Node>,
    commands: &mut Commands,
) -> () {
    let mut stud = node_q.get_mut(slider.stud).unwrap();
    stud.left = Val::Px(percentage * HIGHEST_LEFT - SLIDER_BORDER);

    commands.entity(slider_e).trigger(Slid {
        slid_percentage: percentage,
    });
}
