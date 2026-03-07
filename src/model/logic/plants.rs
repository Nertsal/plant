use super::*;

const GROWTH_TIMER: f32 = 0.05;
const MAX_SIZE: usize = 40;
const SPLIT_CHANCE: f32 = 0.1;

impl Model {
    pub fn update_plants(&mut self, delta_time: Time) {
        let mut rng = thread_rng();

        // Update plants
        let mut to_grow = Vec::new();
        for (idx, plant) in self.grid.plants.iter_mut().enumerate() {
            if plant.growth_timer > Time::ZERO {
                plant.growth_timer -= delta_time;
            } else {
                // Attempt to grow
                plant.growth_timer += r32(GROWTH_TIMER); // TODO: configurable somewhere somehow for different kinds of plants
                if plant.stem.len() > MAX_SIZE {
                    // Stop growing
                    continue;
                }
                to_grow.push(idx);
            }
        }

        // Grow plants
        for grow_idx in to_grow {
            let Some(plant) = self.grid.plants.get(grow_idx) else {
                continue;
            };
            if plant.leaves.is_empty() {
                if plant.stem.is_empty() {
                    // Grow from the root
                    let target = plant.root + vec2(0, 1);
                    if can_grow_into(target, &self.grid) {
                        let plant = self.grid.plants.get_mut(grow_idx).unwrap();
                        plant.leaves.push(target);
                    }
                }
            } else {
                // Grow from the leaves
                #[allow(clippy::type_complexity)]
                let mut growth: Vec<(Option<vec2<ICoord>>, Option<vec2<ICoord>>)> =
                    Vec::with_capacity(plant.leaves.len());
                for &leaf in &plant.leaves {
                    let options: Vec<_> = [vec2(-1, 0), vec2(0, 1), vec2(1, 0)]
                        .iter()
                        .copied()
                        .map(|delta| leaf + delta)
                        .filter(|&pos| {
                            can_grow_into(pos, &self.grid)
                                && !growth
                                    .iter()
                                    .any(|(left, right)| *left == Some(pos) || *right == Some(pos))
                        })
                        .map(|pos| (pos, density_near(pos, &self.grid).recip()))
                        .collect();

                    let split_chance = SPLIT_CHANCE as f64;
                    let grow = if rng.gen_bool(split_chance) {
                        // Split
                        let mut growth = options
                            .choose_multiple_weighted(&mut rng, 2, |(_, w)| *w)
                            .into_iter()
                            .flatten();
                        (
                            growth.next().map(|(p, _)| *p),
                            growth.next().map(|(p, _)| *p),
                        )
                    } else {
                        (
                            options
                                .choose_weighted(&mut rng, |(_, w)| *w)
                                .ok()
                                .map(|(p, _)| *p),
                            None,
                        )
                    };
                    growth.push(grow);
                }

                let plant = self.grid.plants.get_mut(grow_idx).unwrap();
                let mut new_leaves = Vec::new();
                for (leaf, (grow_left, grow_right)) in plant.leaves.iter_mut().zip(growth) {
                    if let Some(grow) = grow_left {
                        plant.stem.push(*leaf);
                        *leaf = grow;
                    }
                    if let Some(grow) = grow_right {
                        new_leaves.push(grow);
                    }
                }
                plant.leaves.extend(new_leaves);
            }
        }
    }
}

pub fn aabb_contains(aabb: Aabb2<ICoord>, pos: vec2<ICoord>) -> bool {
    aabb.min.x <= pos.x && aabb.min.y <= pos.y && aabb.max.x >= pos.x && aabb.max.y >= pos.y
}

pub fn manhattan_distance(a: vec2<ICoord>, b: vec2<ICoord>) -> ICoord {
    (a.x - b.x).abs() + (a.y - b.y).abs()
}

pub fn can_grow_into(pos: vec2<ICoord>, grid: &Grid) -> bool {
    !grid
        .lights
        .iter()
        .any(|light| aabb_contains(light.pos, pos))
        && !grid.plants.iter().any(|plant| {
            itertools::chain![
                [plant.root],
                plant.stem.iter().copied(),
                plant.leaves.iter().copied()
            ]
            .any(|p| p == pos)
        })
}

pub fn density_near(pos: vec2<ICoord>, grid: &Grid) -> f64 {
    const MAX_DISTANCE: ICoord = 5;
    let mut density = 0.0;
    density += grid
        .lights
        .iter()
        .filter(|light| aabb_contains(light.pos, pos))
        .count() as f64;
    density += grid
        .plants
        .iter()
        .flat_map(|plant| {
            itertools::chain![
                [plant.root],
                plant.stem.iter().copied(),
                plant.leaves.iter().copied()
            ]
        })
        .map(|plant| manhattan_distance(plant, pos))
        .filter(|d| *d <= MAX_DISTANCE)
        .map(|d| (d.max(1) as f64).recip())
        .sum::<f64>();
    density / MAX_DISTANCE as f64 / MAX_DISTANCE as f64
}
