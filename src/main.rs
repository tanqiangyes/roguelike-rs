mod components;
mod map;
mod player;
mod rect;
mod state;

use crate::components::{Player, Position, Renderable};
use crate::map::new_map_rooms_and_corridors;
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

    let (rooms, map) = new_map_rooms_and_corridors();
    gs.ecs.insert(map);
    let (player_x, player_y) = rooms[0].center();
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
        .build();

    rltk::main_loop(context, gs)
}
