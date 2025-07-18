use bevy::prelude::*;

const SLIDER_WIDTH: f32 = 200.;
const SLIDER_HEIGHT: f32 = 50.;
const SLIDER_BORDER: f32 = 10.;
const DRAG_START_PLACEHOLDER: f32 = 0.; // Not actually important because it gets immediately overwritten

#[derive(Component)]
#[require(Name::new("Slider"))]
struct Slider {
    drag_start: f32, // in percentage, from 0 to 1
    stud: Entity,
}

#[derive(Event)]
struct Slid {
    val: f32, // From 0.0 to 1.0
}

pub fn create_slider(commands: &mut Commands) -> Entity {
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
            BorderRadius::MAX,
        ))
        .id();

    let stud = commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                width: Val::Px(SLIDER_HEIGHT),
                height: Val::Px(SLIDER_HEIGHT),
                border: UiRect::all(Val::Px(SLIDER_BORDER)),
                left: Val::Px(-SLIDER_BORDER),
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
            |trigger: Trigger<Pointer<DragStart>>, mut slider_q: Query<&mut Slider>| {
                let mut slider = slider_q.get_mut(trigger.target).unwrap();
                slider.drag_start = trigger.hit.position.unwrap().x;
            },
        )
        .observe(
            |trigger: Trigger<Pointer<Drag>>,
             mut node_q: Query<&mut Node>,
             slider_q: Query<&Slider>| {
                let slider = slider_q.get(trigger.target).unwrap();
                // trigger.distance.x is in pixels, not percentage like slider.drag_start
                let mut current_percentage =
                    slider.drag_start + (trigger.distance.x / SLIDER_WIDTH); // From 0 to 1
                current_percentage = current_percentage.min(1.).max(0.);

                // The Val::Px(SLIDER_HEIGHT) is also the width of the stud
                let highest_left = SLIDER_WIDTH - SLIDER_HEIGHT;

                let mut stud = node_q.get_mut(slider.stud).unwrap();
                stud.left = Val::Px(current_percentage * highest_left - SLIDER_BORDER);
            },
        );

    return slider;
}
