mod components;
mod map;
mod monster_ai_system;
mod player;
mod rect;
mod state;
mod visibility_system;
mod map_indexing_system;
mod melee_combat_system;
mod damage_system;
mod gui;
mod gamelog;
mod spawner;

use crate::components::{BlocksTile, CombatStats, Monster, Name, Player, Position, Renderable, SufferDamage, Viewshed, WantsToMelee};
use crate::map::Map;
use crate::state::{RunState, State};
use rltk::{BError, Point, RandomNumberGenerator, RltkBuilder};
use specs::prelude::*;

fn main() -> BError {
    let mut context = RltkBuilder::simple80x50()
        .with_title("roguelike game")
        .build()?;
    context.with_post_scanlines(true);
    let mut gs = State {
        ecs: World::new(),
    };
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();
    gs.ecs.register::<Viewshed>();
    gs.ecs.register::<Monster>();
    gs.ecs.register::<Name>();
    gs.ecs.register::<BlocksTile>();
    gs.ecs.register::<CombatStats>();
    gs.ecs.register::<WantsToMelee>();
    gs.ecs.register::<SufferDamage>();

    let map = Map::new_map_rooms_and_corridors();
    let (player_x, player_y) = map.rooms[0].center();

    let player_entity = spawner::player(&mut gs.ecs, player_x, player_y);

    gs.ecs.insert(RandomNumberGenerator::new());
    for room in map.rooms.iter().skip(1) {
        spawner::spawn_room(&mut gs.ecs, room);
    }
    gs.ecs.insert(map);
    gs.ecs.insert(gamelog::GameLog{ entries: vec!["Welcome to Rusty Roguelike".to_string()] });
    gs.ecs.insert(Point::new(player_x, player_y));
    // player
    gs.ecs.insert(player_entity);
    gs.ecs.insert(RunState::PreRun);
    rltk::main_loop(context, gs)
}
