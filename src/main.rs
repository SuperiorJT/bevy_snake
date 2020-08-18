use bevy::{ecs::Mut, prelude::*};
use std::collections::LinkedList;

const GRID_SIZE: usize = 30;
const GRID_UNIT: f32 = 30.0;

fn main() {
    App::build()
        .add_default_plugins()
        .add_resource(SnakeMovementTimer(Timer::from_seconds(0.5)))
        .add_startup_system(setup.system())
        .add_system(snake_movement_system.system())
        .add_system(player_input_system.system())
        .run();
}

struct SnakeMovementTimer(Timer);

struct KeyBinds {
    up: KeyCode,
    down: KeyCode,
    left: KeyCode,
    right: KeyCode,
}

struct Snake {
    body: LinkedList<Entity>,
    direction: SnakeDirection,
}

enum SnakeDirection {
    Up,
    Down,
    Left,
    Right,
}

struct SnakeHead;
struct SnakeTail;
struct SnakeBody;

struct GridPosition {
    x: i32,
    y: i32,
}
impl GridPosition {
    pub fn new(x: i32, y: i32) -> GridPosition {
        GridPosition { x, y }
    }
}

enum Collider {
    Solid,
    Piece,
    None,
}

fn setup(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    let mut snake_body = LinkedList::new();

    commands.spawn(Camera2dComponents::default());

    snake_body.push_front(
        commands
            .spawn(SpriteComponents {
                material: materials.add(Color::WHITE.into()),
                translation: Translation(Vec3::new(0.0, 0.0, 0.0)),
                sprite: Sprite {
                    size: Vec2::new(GRID_UNIT, GRID_UNIT),
                    ..Default::default()
                },
                ..Default::default()
            })
            .with(SnakeTail)
            .with(GridPosition::new(0, 0))
            .current_entity()
            .unwrap(),
    );

    snake_body.push_front(
        commands
            .spawn(SpriteComponents {
                material: materials.add(Color::WHITE.into()),
                translation: Translation(Vec3::new(0.0, GRID_UNIT, 0.0)),
                sprite: Sprite {
                    size: Vec2::new(GRID_UNIT, GRID_UNIT),
                    ..Default::default()
                },
                ..Default::default()
            })
            .with(SnakeBody)
            .with(GridPosition::new(0, 1))
            .current_entity()
            .unwrap(),
    );

    snake_body.push_front(
        commands
            .spawn(SpriteComponents {
                material: materials.add(Color::WHITE.into()),
                translation: Translation(Vec3::new(0.0, GRID_UNIT * 2.0, 0.0)),
                sprite: Sprite {
                    size: Vec2::new(GRID_UNIT, GRID_UNIT),
                    ..Default::default()
                },
                ..Default::default()
            })
            .with(SnakeHead)
            .with(GridPosition::new(0, 2))
            .current_entity()
            .unwrap(),
    );

    commands
        .spawn((Snake {
            body: snake_body,
            direction: SnakeDirection::Up,
        },))
        .with(KeyBinds {
            up: KeyCode::Up,
            down: KeyCode::Down,
            left: KeyCode::Left,
            right: KeyCode::Right,
        });
}

fn snake_movement_system(
    mut commands: Commands,
    time: Res<Time>,
    mut snake_timer: ResMut<SnakeMovementTimer>,
    mut snake_query: Query<&mut Snake>,
    mut head_query: Query<(&SnakeHead, Entity, &GridPosition, &Translation)>,
    mut tail_query: Query<(&SnakeTail, Entity, &mut GridPosition, &mut Translation)>,
    mut body_query: Query<(&SnakeBody, Entity, &GridPosition, &Translation)>,
) {
    snake_timer.0.tick(time.delta_seconds);
    if !snake_timer.0.finished {
        return;
    }

    for mut snake in &mut snake_query.iter() {
        for (_segment, head_entity, head_grid_pos, head_translation) in &mut head_query.iter() {
            for (_segment, tail_entity, mut grid_pos, mut translation) in &mut tail_query.iter() {
                grid_pos.x = head_grid_pos.x;
                grid_pos.y = head_grid_pos.y;

                *translation.0.x_mut() = head_translation.0.x();
                *translation.0.y_mut() = head_translation.0.y();

                match snake.direction {
                    SnakeDirection::Up => {
                        grid_pos.y += 1;
                        *translation.0.y_mut() += GRID_UNIT
                    }
                    SnakeDirection::Down => {
                        grid_pos.y = grid_pos.y - 1;
                        *translation.0.y_mut() += -GRID_UNIT
                    }
                    SnakeDirection::Left => {
                        grid_pos.x = grid_pos.x - 1;
                        *translation.0.x_mut() += -GRID_UNIT
                    }
                    SnakeDirection::Right => {
                        grid_pos.x += 1;
                        *translation.0.x_mut() += GRID_UNIT
                    }
                }

                commands.remove_one::<SnakeTail>(tail_entity);
                commands.insert_one(tail_entity, SnakeHead);
            }

            commands.remove_one::<SnakeHead>(head_entity);
            commands.insert_one(head_entity, SnakeBody);
        }

        if let Some(e) = snake.body.pop_back() {
            snake.body.push_front(e);
        }

        let tail_entity = snake.body.back().unwrap();

        for (_segment, entity, mut _grid_pos, mut _translation) in &mut body_query.iter() {
            if entity != *tail_entity {
                continue;
            }

            commands.remove_one::<SnakeBody>(entity);
            commands.insert_one(entity, SnakeTail);
        }
    }

    snake_timer.0.reset();
}

fn player_input_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut snake_query: Query<(&mut Snake, &KeyBinds)>,
) {
    for (mut snake, keybinds) in &mut snake_query.iter() {
        if keyboard_input.just_pressed(keybinds.up) {
            if let SnakeDirection::Down = snake.direction {
                return;
            }
            snake.direction = SnakeDirection::Up;
        }
        if keyboard_input.just_pressed(keybinds.down) {
            if let SnakeDirection::Up = snake.direction {
                return;
            }
            snake.direction = SnakeDirection::Down;
        }
        if keyboard_input.just_pressed(keybinds.left) {
            if let SnakeDirection::Right = snake.direction {
                return;
            }
            snake.direction = SnakeDirection::Left;
        }
        if keyboard_input.just_pressed(keybinds.right) {
            if let SnakeDirection::Left = snake.direction {
                return;
            }
            snake.direction = SnakeDirection::Right;
        }
    }
}
