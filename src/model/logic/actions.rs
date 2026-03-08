use super::*;

impl Model {
    pub fn interact_with(&mut self, target: vec2<ICoord>) {
        log::debug!("interact with {}", target);
        let Some(tile) = self.grid.get_tile(target) else {
            // Tell the drone to just fly to this tile
            self.drone.target = DroneTarget::MoveTo(target);
            return;
        };

        self.drone.target = match &tile.tile {
            // Tile::Bug(bug_id) => self.drone.target = DroneTarget::KillBug(bug_id),
            Tile::Leaf(_) => DroneTarget::Interact(target, DroneAction::CutPlant),
            _ => DroneTarget::MoveTo(target),
        };
    }

    pub fn place_tile(&mut self, target: vec2<ICoord>, tile: Tile) -> bool {
        log::debug!("place tile at {}: {:?}", target, tile);
        if self.grid.get_tile(target).is_some() {
            return false;
        }

        let Some(inv_item_idx) = self.inventory.iter().position(|(t, _)| *t == tile) else {
            return false;
        };

        self.grid.set_tile(target, tile);

        if let Some((_, count)) = self.inventory.get_mut(inv_item_idx) {
            if *count > 1 {
                *count -= 1;
            } else {
                self.inventory.remove(inv_item_idx);
            }
        }

        true
    }

    pub fn cut_plant(&mut self, target: vec2<ICoord>) -> bool {
        log::debug!("cut plant at {}", target);
        let Some(tile) = self.grid.get_tile(target) else {
            return false;
        };
        let Tile::Leaf(_) = tile.tile else {
            return false;
        };

        let plant_positions = get_all_connected(&self.grid, target, |tile| {
            matches!(tile.tile, Tile::Leaf(_))
        });

        // Earn money
        let size = plant_positions.len();
        self.money += size as Money;

        // Remove stem and leaves
        for pos in plant_positions {
            if let Some(mut tile) = self.grid.remove_tile(pos)
                && let Tile::Leaf(leaf) = &mut tile.tile
                && leaf.root
            {
                leaf.growth_timer = Some(r32(1.0));
                self.grid.set_tile(pos, tile.tile);
            }
        }

        true
    }

    /// Attempt to plant a seed of a specific kind at the given position.
    /// Returns `true` if planted.
    pub fn plant_seed(&mut self, target: vec2<ICoord>, kind: PlantKind) -> bool {
        log::debug!("plant at {}: {:?}", target, kind);
        if self.grid.get_tile(target).is_some() {
            // Occupied tile
            return false;
        }

        self.grid.set_tile(target, Tile::Leaf(Leaf::new(kind)));
        true
    }
}
