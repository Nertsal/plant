use super::*;

impl Model {
    pub fn cut_plant(&mut self, target: vec2<ICoord>) -> bool {
        log::debug!("cut plant at {}", target);
        let Some(tile) = self.grid.get_tile(target) else {
            return false;
        };
        let Tile::Leaf(_) = tile.tile else {
            return false;
        };

        let mut plant_positions = vec![tile.pos];
        let mut to_check_neighbors: VecDeque<_> = vec![tile].into();

        while let Some(tile) = to_check_neighbors.pop_front() {
            for tile in self.grid.get_neighbors(tile.pos) {
                let Tile::Leaf(_) = tile.tile else { continue };
                plant_positions.push(tile.pos);
                to_check_neighbors.push_back(tile);
            }
        }

        // Earn money
        let size = plant_positions.len();
        self.money += size as Money;

        // Remove stem and leaves
        for pos in plant_positions {
            self.grid.remove_tile(pos);
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
