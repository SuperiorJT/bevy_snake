use bevy::prelude::*;
use rand::Rng;
use std::collections::{HashSet, LinkedList};
mod constants;
mod comp;
mod plugins;

use comp::snake::*;

fn main() {
    App::build()
        .add_default_plugins()
        .add_plugin(plugins::game_state::GameStatePlugin)
        .add_resource(SnakeMovementTimer(Timer::from_seconds(0.3, false)))
        .add_resource(FreeLocations(HashSet::new()))
        .add_resource(PreGameStartListenerState::default())
        .add_resource(PreGameEndListenerState::default())
        .add_resource(RunningGameStartListenerState::default())
        .add_resource(RunningGameEndListenerState::default())
        .add_resource(PostGameEndListenerState::default())
        .add_startup_system(setup.system())
        .add_system(snake_movement_system.system())
        .add_system(player_input_system.system())
        .add_system(snake_collision_system.system())
        // .add_system(debug_food_sprite_system.system())
        .add_system(process_running_start_events.system())
        .add_system(process_pre_start_events.system())
        .add_system(process_pre_end_events.system())
        .add_system(process_post_end_events.system())
        .add_system(process_running_end_events.system())
        .run();
}

struct SnakeMovementTimer(Timer); // make this part of the snek?

struct FreeLocations(HashSet<GridPosition>);

struct KeyBinds {
    up: KeyCode,
    down: KeyCode,
    left: KeyCode,
    right: KeyCode,
}

struct Food;

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
struct GridPosition {
    x: i32,
    y: i32,
}
impl GridPosition {
    pub fn new(x: i32, y: i32) -> GridPosition {
        GridPosition { x, y }
    }
}

#[derive(Default)]
struct RunningGameStartListenerState {
    event_reader: EventReader<plugins::game_state::events::RunningGameStartEvent>
}

fn process_running_start_events(
    mut commands: Commands,
    mut state: ResMut<RunningGameStartListenerState>,
    run_start_events: Res<Events<plugins::game_state::events::RunningGameStartEvent>>,
    mut snake_query: Query<(&Snake, Entity)>
) {
    for _ in state.event_reader.iter(&run_start_events) {
        for (_, e) in &mut snake_query.iter() {
            commands.insert_one(e, comp::Acting);
        }
    }
}

fn setup(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn(Camera2dComponents::default());

    // walls
    let grid_size_float = constants::GRID_SIZE as f32;
    let wall_length = (grid_size_float * 2.0 + 3.0) * constants::GRID_UNIT;
    commands.spawn(SpriteComponents {
        material: materials.add(Color::BLACK.into()),
        translation: Translation(Vec3::new(-(constants::GRID_SIZE + 1) as f32 * constants::GRID_UNIT, 0.0, 0.0)),
        sprite: Sprite {
            size: Vec2::new(constants::GRID_UNIT, wall_length),
            ..Default::default()
        },
        ..Default::default()
    });
    commands.spawn(SpriteComponents {
        material: materials.add(Color::BLACK.into()),
        translation: Translation(Vec3::new((constants::GRID_SIZE + 1) as f32 * constants::GRID_UNIT, 0.0, 0.0)),
        sprite: Sprite {
            size: Vec2::new(constants::GRID_UNIT, wall_length),
            ..Default::default()
        },
        ..Default::default()
    });
    commands.spawn(SpriteComponents {
        material: materials.add(Color::BLACK.into()),
        translation: Translation(Vec3::new(0.0, -(constants::GRID_SIZE + 1) as f32 * constants::GRID_UNIT, 0.0)),
        sprite: Sprite {
            size: Vec2::new(wall_length, constants::GRID_UNIT),
            ..Default::default()
        },
        ..Default::default()
    });
    commands.spawn(SpriteComponents {
        material: materials.add(Color::BLACK.into()),
        translation: Translation(Vec3::new(0.0, (constants::GRID_SIZE + 1) as f32 * constants::GRID_UNIT, 0.0)),
        sprite: Sprite {
            size: Vec2::new(wall_length, constants::GRID_UNIT),
            ..Default::default()
        },
        ..Default::default()
    });
}

fn spawn_game_entities(
    commands: &mut Commands,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    mut free_locations: &mut ResMut<FreeLocations>,
) {
    let mut snake_entity_list = LinkedList::new();
    let tail_pos = GridPosition::new(0, 0);
    let body_pos = GridPosition::new(0, 1);
    let head_pos = GridPosition::new(0, 2);
    let food_pos = GridPosition::new(-3, 2);
    snake_entity_list.push_front(
        commands
            .spawn(SpriteComponents {
                material: materials.add(Color::WHITE.into()),
                translation: Translation(Vec3::new(0.0, 0.0, 0.0)),
                sprite: Sprite {
                    size: Vec2::new(constants::GRID_UNIT, constants::GRID_UNIT),
                    ..Default::default()
                },
                ..Default::default()
            })
            .with(SnakeTail)
            .with(tail_pos)
            .current_entity()
            .unwrap(),
    );

    snake_entity_list.push_front(
        commands
            .spawn(SpriteComponents {
                material: materials.add(Color::WHITE.into()),
                translation: Translation(Vec3::new(0.0, constants::GRID_UNIT, 0.0)),
                sprite: Sprite {
                    size: Vec2::new(constants::GRID_UNIT, constants::GRID_UNIT),
                    ..Default::default()
                },
                ..Default::default()
            })
            .with(SnakeBody)
            .with(body_pos)
            .current_entity()
            .unwrap(),
    );

    snake_entity_list.push_front(
        commands
            .spawn(SpriteComponents {
                material: materials.add(Color::WHITE.into()),
                translation: Translation(Vec3::new(0.0, constants::GRID_UNIT * 2.0, 0.0)),
                sprite: Sprite {
                    size: Vec2::new(constants::GRID_UNIT, constants::GRID_UNIT),
                    ..Default::default()
                },
                ..Default::default()
            })
            .with(SnakeHead)
            .with(head_pos)
            .current_entity()
            .unwrap(),
    );

    commands
        .spawn((Snake {
            body: snake_entity_list,
            direction: SnakeDirection::Up,
            last_direction: SnakeDirection::Up,
        },))
        .with(KeyBinds {
            up: KeyCode::Up,
            down: KeyCode::Down,
            left: KeyCode::Left,
            right: KeyCode::Right,
        });

    commands
        .spawn(SpriteComponents {
            material: materials.add(Color::WHITE.into()),
            translation: Translation(Vec3::new(constants::GRID_UNIT * -3.0, constants::GRID_UNIT * 2.0, 0.0)),
            sprite: Sprite {
                size: Vec2::new(constants::GRID_UNIT / 2.0, constants::GRID_UNIT / 2.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .with(Food)
        .with(food_pos);

    init_free_locations(&mut free_locations);
    free_locations.0.remove(&tail_pos);
    free_locations.0.remove(&body_pos);
    free_locations.0.remove(&head_pos);
    free_locations.0.remove(&food_pos);
}

fn despawn_game_entities(
    commands: &mut Commands,
    snake_entity_list: &LinkedList<Entity>,
    food_entity: Entity,
    snake_entity: Entity,
) {
    snake_entity_list.iter().for_each(|e| {
        commands.despawn(*e);
    });
    commands.despawn(food_entity);
    commands.despawn(snake_entity);
}

fn init_free_locations(free_locations: &mut ResMut<FreeLocations>) {
    free_locations.0.clear();
    for x in -constants::GRID_SIZE..constants::GRID_SIZE {
        for y in -constants::GRID_SIZE..constants::GRID_SIZE {
            free_locations.0.insert(GridPosition::new(x, y));
        }
    }
}

fn snake_movement_system(
    mut commands: Commands,
    time: Res<Time>,
    mut running_end_events: ResMut<Events<plugins::game_state::events::RunningGameEndEvent>>,
    mut snake_timer: ResMut<SnakeMovementTimer>,
    mut free_locations: ResMut<FreeLocations>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut food_query: Query<(&Food, &mut GridPosition, &mut Translation)>,
    mut snake_query: Query<(&mut Snake, &comp::Acting)>,
    mut head_query: Query<(&SnakeHead, Entity, &GridPosition, &Translation)>,
    mut tail_query: Query<(&SnakeTail, Entity, &mut GridPosition, &mut Translation)>,
    mut body_query: Query<(&SnakeBody, Entity, &GridPosition, &Translation)>,
) {
    for (mut snake, _) in &mut snake_query.iter() {
        snake_timer.0.tick(time.delta_seconds);
        if !snake_timer.0.finished {
            return;
        }
        snake_timer.0.reset();
        for (_segment, head_entity, head_grid_pos, head_translation) in &mut head_query.iter() {
            for (_segment, tail_entity, mut grid_pos, mut translation) in &mut tail_query.iter() {
                let mut pending_next_pos = GridPosition::new(head_grid_pos.x, head_grid_pos.y);
                let mut pending_translation = Translation(Vec3::new(
                    head_translation.0.x(),
                    head_translation.0.y(),
                    head_translation.0.z(),
                ));

                match snake.direction {
                    SnakeDirection::Up => {
                        pending_next_pos.y += 1;
                        *pending_translation.0.y_mut() += constants::GRID_UNIT
                    }
                    SnakeDirection::Down => {
                        pending_next_pos.y = pending_next_pos.y - 1;
                        *pending_translation.0.y_mut() += -constants::GRID_UNIT
                    }
                    SnakeDirection::Left => {
                        pending_next_pos.x = pending_next_pos.x - 1;
                        *pending_translation.0.x_mut() += -constants::GRID_UNIT
                    }
                    SnakeDirection::Right => {
                        pending_next_pos.x += 1;
                        *pending_translation.0.x_mut() += constants::GRID_UNIT
                    }
                }

                snake.last_direction = snake.direction;

                for (_segment, _e, body_grid_pos, mut _translation) in &mut body_query.iter() {
                    if *body_grid_pos == pending_next_pos {
                        running_end_events.send(plugins::game_state::events::RunningGameEndEvent);
                        return;
                    }
                }

                for (_food, mut food_pos, mut food_translation) in &mut food_query.iter() {
                    if pending_next_pos == *food_pos {
                        commands.remove_one::<SnakeHead>(head_entity);
                        commands.insert_one(head_entity, SnakeBody);

                        let head_pos = food_pos.clone();
                        let new_pos = get_random_location(&free_locations);

                        food_pos.x = new_pos.x;
                        food_pos.y = new_pos.y;

                        *food_translation.0.x_mut() = constants::GRID_UNIT * food_pos.x as f32;
                        *food_translation.0.y_mut() = constants::GRID_UNIT * food_pos.y as f32;

                        println!("{:?}, {:?}", food_pos, food_translation.0);

                        free_locations.0.remove(&*food_pos);

                        snake.body.push_front(
                            commands
                                .spawn(SpriteComponents {
                                    material: materials.add(Color::WHITE.into()),
                                    translation: Translation(Vec3::new(
                                        constants::GRID_UNIT * head_pos.x as f32,
                                        constants::GRID_UNIT * head_pos.y as f32,
                                        0.0,
                                    )),
                                    sprite: Sprite {
                                        size: Vec2::new(constants::GRID_UNIT, constants::GRID_UNIT),
                                        ..Default::default()
                                    },
                                    ..Default::default()
                                })
                                .with(SnakeHead)
                                .with(head_pos)
                                .current_entity()
                                .unwrap(),
                        );
                    } else {
                        free_locations
                            .0
                            .insert(GridPosition::new(grid_pos.x, grid_pos.y));

                        grid_pos.x = pending_next_pos.x;
                        grid_pos.y = pending_next_pos.y;

                        *translation.0.x_mut() = pending_translation.0.x();
                        *translation.0.y_mut() = pending_translation.0.y();

                        free_locations.0.remove(&*grid_pos);

                        commands.remove_one::<SnakeTail>(tail_entity);
                        commands.insert_one(tail_entity, SnakeHead);

                        commands.remove_one::<SnakeHead>(head_entity);
                        commands.insert_one(head_entity, SnakeBody);

                        if let Some(e) = snake.body.pop_back() {
                            snake.body.push_front(e);
                        }

                        let tail_entity = snake.body.back().unwrap();

                        for (_segment, entity, mut _grid_pos, mut _translation) in
                            &mut body_query.iter()
                        {
                            if entity != *tail_entity {
                                continue;
                            }

                            commands.remove_one::<SnakeBody>(entity);
                            commands.insert_one(entity, SnakeTail);
                        }
                    }
                }
            }
        }
    }
}

fn player_input_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut snake_query: Query<(&mut Snake, &KeyBinds)>,
) {
    for (mut snake, keybinds) in &mut snake_query.iter() {
        if keyboard_input.just_pressed(keybinds.up) {
            if let SnakeDirection::Down = snake.last_direction {
                return;
            }
            snake.direction = SnakeDirection::Up;
        }
        if keyboard_input.just_pressed(keybinds.down) {
            if let SnakeDirection::Up = snake.last_direction {
                return;
            }
            snake.direction = SnakeDirection::Down;
        }
        if keyboard_input.just_pressed(keybinds.left) {
            if let SnakeDirection::Right = snake.last_direction {
                return;
            }
            snake.direction = SnakeDirection::Left;
        }
        if keyboard_input.just_pressed(keybinds.right) {
            if let SnakeDirection::Left = snake.last_direction {
                return;
            }
            snake.direction = SnakeDirection::Right;
        }
    }
}

fn snake_collision_system(
    mut run_end_events: ResMut<Events<plugins::game_state::events::RunningGameEndEvent>>,
    mut snake_query: Query<(&Snake, &comp::Acting)>,
    mut head_query: Query<(&SnakeHead, &GridPosition)>,
) {
    for (_, _) in &mut snake_query.iter() {
        for (_head, pos) in &mut head_query.iter() {
            if pos.x > constants::GRID_SIZE || pos.x < -constants::GRID_SIZE || pos.y > constants::GRID_SIZE || pos.y < -constants::GRID_SIZE {
                run_end_events.send(plugins::game_state::events::RunningGameEndEvent);
            }
        }
    }
}

#[derive(Default)]
struct RunningGameEndListenerState {
    event_reader: EventReader<plugins::game_state::events::RunningGameEndEvent>
}

fn process_running_end_events(
    mut commands: Commands,
    mut state: ResMut<RunningGameEndListenerState>,
    running_end_events: Res<Events<plugins::game_state::events::RunningGameEndEvent>>,
    mut snake_query: Query<(&Snake, Entity)>,
) {
    for _ in state.event_reader.iter(&running_end_events) {
        for (_, e) in &mut snake_query.iter() {
            commands.remove_one::<comp::Acting>(e);
        }
    }
}

#[derive(Default)]
struct PostGameEndListenerState {
    event_reader: EventReader<plugins::game_state::events::PostGameEndEvent>
}

fn process_post_end_events(
    mut commands: Commands,
    mut state: ResMut<PostGameEndListenerState>,
    post_end_events: Res<Events<plugins::game_state::events::PostGameEndEvent>>,
    mut snake_query: Query<(&Snake, Entity)>,
    mut food_query: Query<(&Food, Entity)>,
) {
    for _ in state.event_reader.iter(&post_end_events) {
        for (_food, food_entity) in &mut food_query.iter() {
            for (snake, snake_entity) in &mut snake_query.iter() {
                despawn_game_entities(&mut commands, &snake.body, food_entity, snake_entity);
            }
        }
    }
}

#[derive(Default)]
struct PreGameStartListenerState {
    event_reader: EventReader<plugins::game_state::events::PreGameStartEvent>
}

fn process_pre_start_events(
    mut commands: Commands,
    mut state: ResMut<PreGameStartListenerState>,
    pre_start_events: Res<Events<plugins::game_state::events::PreGameStartEvent>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut free_locations: ResMut<FreeLocations>
) {
    for _ in state.event_reader.iter(&pre_start_events) {
        spawn_game_entities(&mut commands, &mut materials, &mut free_locations);
    }
}

#[derive(Default)]
struct PreGameEndListenerState {
    event_reader: EventReader<plugins::game_state::events::PreGameEndEvent>
}

fn process_pre_end_events(
    mut state: ResMut<PreGameEndListenerState>,
    pre_end_events: Res<Events<plugins::game_state::events::PreGameEndEvent>>,
    mut snake_timer: ResMut<SnakeMovementTimer>,
) {
    for _ in state.event_reader.iter(&pre_end_events) {
        snake_timer.0.reset();
    }
}

fn debug_food_sprite_system(time: Res<Time>, mut query: Query<(&Food, &mut Translation)>) {
    for (_food, mut translation) in &mut query.iter() {
        *translation.0.x_mut() += 30.0 * time.delta_seconds;
    }
}

fn get_random_location(locations: &ResMut<FreeLocations>) -> GridPosition {
    let index = rand::thread_rng().gen_range(0, locations.0.len());
    *locations.0.iter().nth(index).unwrap()
}
