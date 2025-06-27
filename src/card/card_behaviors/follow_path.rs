
use bevy::{
    color::palettes::css::YELLOW,
    ecs::{component::HookContext, world::DeferredWorld},
    prelude::*,
};
use pathfinding::prelude::astar;

use crate::{
    debug::get_debug_env_var,
    global::{
        get_entire_map_rect, get_left_river_rect, get_middle_river_rect, get_right_river_rect,
        GameState,
    },
    manage_level::IsPaused,
};

#[derive(Component)]
#[require(Transform)]
#[component(on_insert = generate_path)]
pub struct FollowPath {
    goal: (i32, i32),
    path: Vec<Vec2>,
    current: usize,
    speed: f32,
}

pub fn follow_path_plugin(app: &mut App) {
    app.add_systems(
        FixedUpdate,
        follow_paths.run_if(in_state(GameState::InGame).and(in_state(IsPaused::False))),
    );

    if get_debug_env_var() {
        app.add_systems(FixedUpdate, display_paths);
    }
}

impl FollowPath {
    pub fn new(goal: (i32, i32), speed: f32) -> Self {
        FollowPath {
            goal,
            speed,
            path: Vec::default(),
            current: usize::default(),
        }
    }

    pub fn get_goal(&self) -> (i32, i32) {
        self.goal
    }
}

pub fn follow_paths(path_followers: Query<(&mut Transform, &mut FollowPath)>, time: Res<Time>) {
    for (mut transform, mut follow_path) in path_followers {
        const TOLERANCE: f32 = 1.0;
        let stop = follow_path.path[follow_path.current];

        if stop.distance(transform.translation.truncate()) < TOLERANCE
            && follow_path.current < follow_path.path.len() - 1
        {
            follow_path.current += 1;
        } else if stop.distance(transform.translation.truncate()) >= TOLERANCE {
            let mut to = stop - transform.translation.truncate();
            to = to.normalize_or_zero();
            transform.translation += (to * follow_path.speed * time.delta_secs()).extend(0.0);
        }
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Pos(pub i32, pub i32);

const ASTAR_RESOLUTION: i32 = 15;
impl Pos {
    fn distance(&self, other: &Pos) -> u32 {
        let a = self.0 - other.0;
        let b = self.1 - other.1;
        ((a * a + b * b) as f32).sqrt() as u32
    }

    fn successors(&self) -> Vec<(Pos, u32)> {
        let straight_cost: u32 = 100;
        let diagonal_cost: u32 = 141; // sqrt(2)

        let &Pos(x, y) = self;
        vec![
            (Pos(x + ASTAR_RESOLUTION, y), straight_cost),
            (Pos(x - ASTAR_RESOLUTION, y), straight_cost),
            (Pos(x, y + ASTAR_RESOLUTION), straight_cost),
            (Pos(x, y - ASTAR_RESOLUTION), straight_cost),
            (
                Pos(x + ASTAR_RESOLUTION, y + ASTAR_RESOLUTION),
                diagonal_cost,
            ),
            (
                Pos(x + ASTAR_RESOLUTION, y - ASTAR_RESOLUTION),
                diagonal_cost,
            ),
            (
                Pos(x - ASTAR_RESOLUTION, y + ASTAR_RESOLUTION),
                diagonal_cost,
            ),
            (
                Pos(x - ASTAR_RESOLUTION, y - ASTAR_RESOLUTION),
                diagonal_cost,
            ),
        ]
        .into_iter()
        .filter(|(p, _)| reachable(p))
        .collect()
    }
}

impl Into<Vec2> for Pos {
    fn into(self) -> Vec2 {
        Vec2::new(self.0 as f32, self.1 as f32)
    }
}

impl From<(i32, i32)> for Pos {
    fn from(item: (i32, i32)) -> Self {
        Pos(item.0, item.1)
    }
}

fn generate_path(mut world: DeferredWorld, context: HookContext) {
    let start = world
        .get::<Transform>(context.entity)
        .unwrap()
        .translation
        .clone();

    let mut follow_path = world.get_mut::<FollowPath>(context.entity).unwrap();

    let goal = follow_path.goal;

    let found_path = astar(
        &Pos(start.x as i32, start.y as i32),
        |p| p.successors(),
        |p| p.distance(&goal.into()) / 3,
        |p| p.distance(&goal.into()) <= ASTAR_RESOLUTION.try_into().unwrap(),
    );

    let Some((found_path, _)) = found_path else {
        panic!("Tried to generate an impossible path to {:?}", goal);
    };

    debug_assert!(follow_path.path.is_empty());
    for pos in found_path {
        follow_path.path.push(pos.into());
    }
}

fn display_paths(paths: Query<&FollowPath>, mut draw: Gizmos) {
    for path in paths {
        for point in path.path.clone() {
            draw.circle_2d(Isometry2d::from_translation(point), 1.0, YELLOW);
        }
    }
}

fn reachable(pos: &Pos) -> bool {
    let pos: Vec2 = Vec2::new(pos.0 as f32, pos.1 as f32);

    get_entire_map_rect().contains(pos)
        && !get_left_river_rect().contains(pos)
        && !get_middle_river_rect().contains(pos)
        && !get_right_river_rect().contains(pos)
}
