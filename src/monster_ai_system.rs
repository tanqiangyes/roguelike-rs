use crate::components::{Monster, Name, Viewshed};
use rltk::{console, Point};
use specs::prelude::*;

pub struct MonitorAi {}

impl<'a> System<'a> for MonitorAi {
    type SystemData = (
        ReadExpect<'a, Point>,
        ReadStorage<'a, Viewshed>,
        ReadStorage<'a, Monster>,
        ReadStorage<'a, Name>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (player_point, viewshed, monster, name) = data;
        for (viewshed, _monster, name) in (&viewshed, &monster, &name).join() {
            if viewshed.visible_tiles.contains(&*player_point) {
                console::log(&format!("{} shouts insults", name.name));
            }
        }
    }
}
