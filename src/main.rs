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

use crate::components::{BlocksTile, CombatStats, Monster, Name, Player, Position, Renderable, SufferDamage, Viewshed, WantsToMelee};
use crate::map::Map;
use crate::state::{RunState, State};
use rltk::{BError, Point, RandomNumberGenerator, RltkBuilder, RGB};
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

    let mut rng = RandomNumberGenerator::new();

    let player_entity = gs.ecs
        .create_entity()
        .with(Position { x: player_x, y: player_y })
        .with(Renderable {
            glyph: rltk::to_cp437('@'),
            fg: RGB::named(rltk::YELLOW),
            bg: RGB::named(rltk::BLACK),
        })
        .with(Player{})
        .with(Viewshed{ visible_tiles : Vec::new(), range: 8, dirty: true })
        .with(Name{name: "Player".to_string() })
        .with(CombatStats{ max_hp: 30, hp: 30, defense: 2, power: 5 })
        .build();

    for (i, room) in map.rooms.iter().skip(1).enumerate() {
        let (x, y) = room.center();
        let glyph: rltk::FontCharType;
        let name: String;
        let roll = rng.roll_dice(1, 2);
        match roll {
            1 => {
                glyph = rltk::to_cp437('g');
                name = "Goblin".to_string();
            }
            _ => {
                glyph = rltk::to_cp437('o');
                name = "Orc".to_string();
            }
        }
        // npc
        gs.ecs
            .create_entity()
            .with(Position { x, y })
            .with(Renderable {
                glyph,
                fg: RGB::named(rltk::RED),
                bg: RGB::named(rltk::BLACK),
            })
            .with(Viewshed {
                visible_tiles: Vec::new(),
                range: 8,
                dirty: true,
            })
            .with(Monster {})
            .with(Name {
                name: format!("{}_num_{}", &name, i),
            })
            .with(BlocksTile{})
            .with(CombatStats{
                max_hp: 16,
                hp: 16,
                defense: 1,
                power: 4,
            })
            .build();
    }
    gs.ecs.insert(map);
    gs.ecs.insert(gamelog::GameLog{ entries: vec!["Welcome to Rusty Roguelike".to_string()] });
    gs.ecs.insert(Point::new(player_x, player_y));
    // player
    gs.ecs.insert(player_entity);
    gs.ecs.insert(RunState::PreRun);
    rltk::main_loop(context, gs)
}
