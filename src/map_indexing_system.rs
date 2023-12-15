use specs::{Join, ReadStorage, System, WriteExpect};
use super::{Map,BlocksTile, Position};

pub struct  MapIndexingSystem {}

impl<'a>  System<'a> for  MapIndexingSystem {
    type SystemData = (
        WriteExpect<'a, Map>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, BlocksTile>
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut map, positions, blocks)= data;
        map.populate_blocked();
        for (position, _blocks) in (&positions, &blocks).join() {
            let idx = map.xy_idx(position.x, position.y);
            map.blocked[idx] = true;
        }
    }
}