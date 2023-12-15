use crate::components::{Monster, Name, Position, Viewshed};
use rltk::{console, Point};
use specs::prelude::*;
use crate::map::Map;

pub struct MonitorAi {}

impl<'a> System<'a> for MonitorAi {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        WriteExpect<'a, Map>,
        ReadExpect<'a, Point>,
        WriteStorage<'a, Viewshed>,
        ReadStorage<'a, Monster>,
        ReadStorage<'a, Name>,
        WriteStorage<'a, Position>
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut map,player_point, mut viewshed, monster, name, mut position) = data;
        for (mut viewshed, _monster, name, mut pos) in (&mut viewshed, &monster, &name, &mut position).join() {
            if viewshed.visible_tiles.contains(&*player_point) {
                let distance = rltk::DistanceAlg::Pythagoras.distance2d(Point::new(pos.x,pos.y), *player_point);
                if distance < 1.5 {
                    console::log(&format!("{} shouts insults", name.name));
                    return;
                }

                let path = rltk::a_star_search(
                    map.xy_idx(pos.x, pos.y) as i32,
                    map.xy_idx(player_point.x, player_point.y)as i32,
                    &mut *map
                );
                if path.success && path.steps.len() > 1 {
                    pos.x = path.steps[1] as i32 % map.width;
                    pos.y = path.steps[1] as i32 / map.width;
                    viewshed.dirty = true;
                }
            }
        }
    }
}
