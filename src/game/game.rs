use crate::game::BuildingId;
use crate::game::Character;
use crate::game::CharacterMobility;
use crate::game::ColonyShipLanding;
use crate::game::Entities;
use crate::game::Entity;
use crate::game::Farm;
use crate::game::Game;
use crate::game::MovableUnit;
use crate::game::Nation;
use crate::game::Warehouse;
use crate::geometry::circle::Circle;
use crate::geometry::circle_rectangle;
use nalgebra::Point2;
use ordered_float::OrderedFloat;
use rand::prelude::SliceRandom;
use std::collections::HashSet;

pub const CITY_RADIUS: f32 = 10.;
pub const CITY_RADIUS_SQUARED: f32 = CITY_RADIUS * CITY_RADIUS;

impl Entities {
    pub fn unit_comparator_by_x(&self, a: usize, b: usize) -> std::cmp::Ordering {
        let characters = &self.characters;

        let a_x = OrderedFloat(characters[a].get_x());
        let b_x = OrderedFloat(characters[b].get_x());
        a_x.cmp(&b_x)
    }

    pub fn unit_comparator_by_y(&self, a: usize, b: usize) -> std::cmp::Ordering {
        let characters = &self.characters;

        let a_y = OrderedFloat(characters[a].get_y());
        let b_y = OrderedFloat(characters[b].get_y());
        a_y.cmp(&b_y)
    }
}

impl Entity for Character {
    fn get_x(&self) -> f32 {
        match self.mobility {
            CharacterMobility::Unit { body, .. } => body.center.x,
            CharacterMobility::Parked { .. } => {
                unreachable!("caller should filter out parked character")
            }
        }
    }

    fn get_y(&self) -> f32 {
        match self.mobility {
            CharacterMobility::Unit { body, .. } => body.center.y,
            CharacterMobility::Parked { .. } => {
                unreachable!("caller should filter out parked character")
            }
        }
    }
}

// 3d rectangular prism intersection test https://stackoverflow.com/a/6008261

impl MovableUnit for Character {
    fn cal_speed(&self) -> f32 {
        10.
    }

    fn cal_max_steering_angle(&self) -> f32 {
        std::f32::consts::PI / 12. // 15 degree
    }
}

impl Game {
    pub fn is_colonized(&self, node_idx: usize) -> bool {
        !self.nodes[node_idx].buildings.is_empty()
    }

    fn can_build_at(&self, top_left: [u32; 2], width: u32, height: u32) -> bool {
        //
        let [tl_x, tl_y] = top_left;

        if let Some(target_node_idx) = self.search_node_by_coor([tl_x as f32, tl_y as f32]) {
            if self.are_tiles_in_same_node(top_left, target_node_idx, width, height) {
                for x in 0..width {
                    let x = x + tl_x;
                    for y in 0..height {
                        let y = y + tl_y;
                        if self.building_grid.contains_key(&(x, y)) {
                            return false;
                        }
                    }
                }
                return true;
            }
        }

        false
    }

    fn search_node_by_coor(&self, [x, y]: [f32; 2]) -> Option<usize> {
        for (idx, node) in self.nodes.iter().enumerate() {
            let coor = node.coor;
            let [dx, dy] = [coor.x - x, coor.y - y];
            let dist = dx * dx + dy * dy;

            if dist < CITY_RADIUS_SQUARED {
                return Some(idx);
            }
        }

        None
    }

    pub fn colonize(&mut self) -> Option<BuildingId> {
        // find an uncolonized node
        // add a nation tag
        // add the landing building, starting population and elites
        // randomize elites' relationship
        // elect a leader based on charisma & relationship

        let uncolonized_nodes: Vec<_> = (0..self.nodes.len())
            .filter(|&i| self.nodes[i].buildings.is_empty())
            .collect();

        if uncolonized_nodes.is_empty() {
            panic!("panic for now");
            // return None;
        }

        let &node_idx = uncolonized_nodes
            .choose(&mut self.rng)
            .expect("slice already checked to be unempty");

        let ret = self.place_colony_ship_landing(node_idx);
        assert!(ret.is_some());

        // random generate a nation for now(?)

        // wanted: create "colony ship" (not physically in game) with starting information and then pass information here

        let nation = Nation {
            cities: Default::default(),
        };

        self.nations.push(nation);

        // create people

        let num_starting_colonists = 10;

        for _ in 0..num_starting_colonists {
            self.entities.characters.push(Character {
                children: Default::default(),
                charisma: 0,
                dexterity: 0,
                strength: 0,
                intelligence: 0,
                mobility: CharacterMobility::Parked { node_idx },
            });
        }

        ret
    }

    pub fn place_colony_ship_landing(&mut self, node_idx: usize) -> Option<BuildingId> {
        const WIDTH: u32 = 3;
        const HEIGHT: u32 = 3;
        assert!(!self.is_colonized(node_idx));

        let center_f32 = self.nodes[node_idx].coor;
        let [x, y] = [center_f32.x, center_f32.y];
        let [xu32, yu32] = [x as u32, y as u32];
        let top_left_u32 = {
            let x = if xu32 == 0 { 0 } else { xu32 - 1 };
            let y = if yu32 == 0 { 0 } else { yu32 - 1 };
            [x, y]
        };
        let top_left_f32 = [top_left_u32[0] as f32, top_left_u32[1] as f32];

        if self.can_build_at(top_left_u32, WIDTH, HEIGHT) {
            if let Some(target_node_idx) = self.search_node_by_coor(top_left_f32) {
                if self.are_tiles_in_same_node(top_left_u32, target_node_idx, WIDTH, HEIGHT) {
                    let landing = ColonyShipLanding {};
                    let buildings = &mut self.entities.buildings;
                    let building_id = BuildingId::ColonyShipLanding(buildings.landings.len());
                    buildings.landings.push(landing);

                    self.bulk_insert_building_grid(top_left_u32, building_id, WIDTH, HEIGHT);

                    let is_inserted = self.nodes[target_node_idx].buildings.insert(building_id);
                    assert!(is_inserted);

                    return Some(building_id);
                }
            }
        }

        None
    }

    fn are_tiles_in_same_node(
        &self,
        top_left: [u32; 2],
        target_node_idx: usize,
        width: u32,
        height: u32,
    ) -> bool {
        let [tl_x, tl_y] = top_left;
        for x in 0..width {
            let x = x + tl_x;
            for y in 0..height {
                let y = y + tl_y;
                if let Some(node_idx) = self.search_node_by_coor([x as f32, y as f32]) {
                    if node_idx == target_node_idx {
                        // (x,y) *is* not part of the node
                        continue;
                    }
                // (x,y) is not part of the node
                } else {
                    // (x,y) is not part of the node
                }
                // (x,y) is not part of the node
                return false;
            }
        }
        true
    }

    fn bulk_insert_building_grid(
        &mut self,
        top_left: [u32; 2],
        building_id: BuildingId,
        width: u32,
        height: u32,
    ) {
        let [tl_x, tl_y] = top_left;
        for x in 0..width {
            let x = x + tl_x;
            for y in 0..height {
                let y = y + tl_y;
                let prev_value = self.building_grid.insert((x, y), building_id);
                assert!(prev_value.is_none());
            }
        }
    }

    pub fn create_farm(&mut self, top_left: [u32; 2]) -> Option<BuildingId> {
        let [tl_x, tl_y] = top_left;
        const WIDTH: u32 = 2;
        const HEIGHT: u32 = 2;

        if self.can_build_at(top_left, WIDTH, HEIGHT) {
            if let Some(target_node_idx) = self.search_node_by_coor([tl_x as f32, tl_y as f32]) {
                if self.are_tiles_in_same_node(top_left, target_node_idx, WIDTH, HEIGHT) {
                    let farm = Farm { output: 0 };
                    let buildings = &mut self.entities.buildings;
                    let building_id = BuildingId::Farm(buildings.farms.len());
                    buildings.farms.push(farm);

                    self.bulk_insert_building_grid(top_left, building_id, WIDTH, HEIGHT);

                    let is_inserted = self.nodes[target_node_idx].buildings.insert(building_id);
                    assert!(is_inserted);

                    return Some(building_id);
                }
            }
        }

        None
    }

    pub fn create_warehouse(&mut self, top_left: [u32; 2]) -> Option<BuildingId> {
        let [tl_x, tl_y] = top_left;
        const WIDTH: u32 = 2;
        const HEIGHT: u32 = 2;

        if self.can_build_at(top_left, WIDTH, HEIGHT) {
            if let Some(target_node_idx) = self.search_node_by_coor([tl_x as f32, tl_y as f32]) {
                if self.are_tiles_in_same_node(top_left, target_node_idx, WIDTH, HEIGHT) {
                    let warehouse = Warehouse::default();
                    let buildings = &mut self.entities.buildings;
                    let building_id = BuildingId::Warehouse(buildings.warehouses.len());
                    buildings.warehouses.push(warehouse);

                    self.bulk_insert_building_grid(top_left, building_id, WIDTH, HEIGHT);

                    let is_inserted = self.nodes[target_node_idx].buildings.insert(building_id);
                    assert!(is_inserted);

                    return Some(building_id);
                }
            }
        }

        None
    }

    pub fn get_unit_coor(&self, idx: usize) -> Point2<f32> {
        let character = &self.entities.characters[idx];

        match character.mobility {
            CharacterMobility::Parked { .. } => {
                unreachable!("only deployed characters should be part of the index")
            }
            CharacterMobility::Unit { body, .. } => body.center,
        }
    }

    pub fn estimate_nearby_units(&self, target: Point2<f32>, radius: f32) -> HashSet<usize> {
        const DELTA: f32 = 0.1;
        const MAX_WIDTH: f32 = 2. + DELTA; // over-estimate bounds for binary search
        const MAX_HEIGHT: f32 = 2. + DELTA;

        let (target_x, target_y) = (target.x, target.y);
        let (x_left, x_right) = (
            OrderedFloat(target_x - radius - MAX_WIDTH),
            OrderedFloat(target_x + radius + MAX_WIDTH),
        );
        let (y_left, y_right) = (
            OrderedFloat(target_y - radius - MAX_HEIGHT),
            OrderedFloat(target_y + radius + MAX_HEIGHT),
        );

        let x_lower_bound = match self.unit_x_axis.binary_search_by(|idx| {
            OrderedFloat(self.entities.characters[*idx].get_x()).cmp(&x_left)
        }) {
            Ok(idx) => idx,
            Err(idx) => idx,
        };

        let x_upper_bound = match self.unit_x_axis.binary_search_by(|idx| {
            OrderedFloat(self.entities.characters[*idx].get_x()).cmp(&x_right)
        }) {
            Ok(idx) => idx,
            Err(idx) => idx,
        };

        assert!(x_lower_bound <= x_upper_bound);

        let y_lower_bound = match self.unit_y_axis.binary_search_by(|idx| {
            OrderedFloat(self.entities.characters[*idx].get_y()).cmp(&y_left)
        }) {
            Ok(idx) => idx,
            Err(idx) => idx,
        };

        let y_upper_bound = match self.unit_y_axis.binary_search_by(|idx| {
            OrderedFloat(self.entities.characters[*idx].get_y()).cmp(&y_right)
        }) {
            Ok(idx) => idx,
            Err(idx) => idx,
        };

        println!(
            "target:{}, radius:{}, x_low:{}, x_high:{}, y_low:{}, y_high:{}",
            target, radius, x_lower_bound, x_upper_bound, y_lower_bound, y_upper_bound
        );

        assert!(y_lower_bound <= y_upper_bound);

        let (x_len, y_len) = (x_upper_bound - x_lower_bound, y_upper_bound - y_lower_bound);

        if x_len == 0 || y_len == 0 {
            Default::default()
        } else {
            let mut xs: HashSet<usize> = HashSet::with_capacity(x_len);
            xs.extend(self.unit_x_axis[x_lower_bound..x_upper_bound].iter());

            let mut ys: HashSet<usize> = HashSet::with_capacity(y_len);
            ys.extend(self.unit_y_axis[y_lower_bound..y_upper_bound].iter());

            let search_area = Circle {
                center: target,
                radius,
            };

            xs.intersection(&ys)
                .cloned()
                .filter(|idx| {
                    // refine search

                    if let CharacterMobility::Unit { body, .. } =
                        self.entities.characters[*idx].mobility
                    {
                        circle_rectangle::is_intersect(&body, &search_area)
                    } else {
                        unreachable!("undeployed characters shouldn't be part of the index");
                    }
                })
                .collect()
        }
    }

    pub fn set_unit_destination(&mut self, character_idx: usize, city_idx: usize) {
        assert!(character_idx < self.entities.characters.len());
        assert!(city_idx < self.nodes.len());

        // TODO remove me later
        {
            let character = &self.entities.characters[character_idx];
            match character.mobility {
                CharacterMobility::Parked { .. } => unimplemented!("TODO not right now"),
                CharacterMobility::Unit { .. } => {
                    // Ok
                }
            }
        }

        self.unit_destinations.insert(character_idx, city_idx);
    }
}
