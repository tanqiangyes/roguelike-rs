mod components;
mod map;
mod player;
mod rect;
mod state;
mod visibility_system;

use crate::components::{Player, Position, Renderable, Viewshed};
use crate::map::Map;
use crate::state::State;
use rltk::{BError, RltkBuilder, RGB};
use specs::prelude::*;

fn main() -> BError {
    let context = RltkBuilder::simple80x50()
        .with_title("roguelike game")
        .build()?;
    let mut gs = State { ecs: World::new() };
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();
    gs.ecs.register::<Viewshed>();

    let map = Map::new_map_rooms_and_corridors();
    let (player_x, player_y) = map.rooms[0].center();
    gs.ecs.insert(map);
    gs.ecs
        .create_entity()
        .with(Position {
            x: player_x,
            y: player_y,
        })
        .with(Renderable {
            glyph: rltk::to_cp437('@'),
            fg: RGB::named(rltk::YELLOW),
            bg: RGB::named(rltk::BLACK),
        })
        .with(Player {})
        .with(Viewshed {
            visible_tiles: Vec::new(),
            range: 8,
            dirty: true,
        })
        .build();

    rltk::main_loop(context, gs)
}
