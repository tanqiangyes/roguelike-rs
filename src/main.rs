use rltk::{BError, GameState, Rltk, RltkBuilder};
fn main() -> BError {
    let context = RltkBuilder::simple80x50()
        .with_title("roguelike game")
        .build()?;
    let gs = State {};
    rltk::main_loop(context, gs)
}

struct State {}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();
        ctx.print(1, 1, "Hello roguelike game!");`
    }
}
