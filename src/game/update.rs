use crate::algorithm::insertion_sort::insertion_sort;
use crate::game::CharacterMobility;
use crate::game::Game;
use crate::game::MovableUnit;
use crate::game::Projectile;
use nalgebra::{Point2, Vector2};
use std::collections::HashSet;

impl Game {
    pub fn cal_computer_decisions(&mut self) {
        /*
        for all people
            make national decisions if person is a leader
                - declare war
            make strategic decisions if person is a general
                - set up/remove squads
                - rebase squad if necessary
                - set rally point
                - set target
            make tactical decisions if person is a squad leader
                * follow path (pathfinding from rally point orders)
                * or resupply from nodes
                * or go home
                * or run away from combat

            split between short-term (per every few ticks) & long-term planning (per few turns)
        */
    }

    pub fn update_state(&mut self) {
        self.cal_computer_decisions();

        // update entities' location
        let entities = &mut self.entities;

        let unit_destinations = &mut self.unit_destinations;
        let nodes = &self.nodes;

        unit_destinations.retain(|unit_idx, &mut node_idx| {
            let character = &mut entities.characters[*unit_idx];
            let speed = character.cal_speed();
            let node_coor = nodes[node_idx].coor;

            match &mut character.mobility {
                CharacterMobility::Unit { body, .. } => {
                    let coor = body.center;
                    let distance_squared = nalgebra::distance_squared(&coor, &node_coor);

                    let speed_squared = speed * speed;

                    if distance_squared < speed_squared {
                        // arrival
                        character.mobility = CharacterMobility::Parked { node_idx };
                        false
                    } else {
                        let from = Vector2::new(coor.x, coor.y);

                        let displacement = (node_coor - coor).normalize() * speed;

                        let new_position: Point2<_> = (from + displacement).into();

                        body.center = new_position;

                        // if enemy units are within range (say, r) of the longest weapon, fire at them
                        // 1. search for units within "r", filter by hostility; called this collection of units "C"
                        // 2. turn "C" into a binary heap, sorted by distance in ascending order, called "H"
                        // 2.1. for each weapon, if top of "H" is in range
                        // 2.1.1 if weapon doesn't have a target, assign the target
                        // 2.1.2 if weapon has a target but the new target has lower HP, assign the new target
                        // 2.2 repeat until all weapon have a target or no more target in range
                        // 2.3 fire all weapons

                        true
                    }
                }
                _ => unreachable!("character isn't deployed"),
            }
        });

        for projectile in &mut entities.projectiles {}

        // TODO filter out projectiles that are still moving (all 3 projectile loops should be unified into 1)

        // move projectiles
        // collision detection
        // calculate damage and destruction
        // fire projectiles from units

        {
            {
                let xs: HashSet<_> = self.unit_x_axis.iter().cloned().collect();
                let ys: HashSet<_> = self.unit_y_axis.iter().cloned().collect();
                dbg!(xs.len() == self.unit_x_axis.len());
                dbg!(ys.len() == self.unit_y_axis.len());

                assert!(xs == ys);
            }

            self.unit_x_axis
                .retain(|&idx| match entities.characters[idx].mobility {
                    CharacterMobility::Parked { .. } => false,
                    CharacterMobility::Unit { .. } => true,
                });
            self.unit_y_axis
                .retain(|&idx| match entities.characters[idx].mobility {
                    CharacterMobility::Parked { .. } => false,
                    CharacterMobility::Unit { .. } => true,
                });

            {
                let xs: HashSet<_> = self.unit_x_axis.iter().cloned().collect();
                let ys: HashSet<_> = self.unit_y_axis.iter().cloned().collect();
                dbg!(&xs);
                dbg!(&ys);

                assert!(xs == ys);
            }

            insertion_sort(&mut self.unit_x_axis, |a, b| {
                entities.unit_comparator_by_x(*a, *b)
            });
            insertion_sort(&mut self.unit_y_axis, |a, b| {
                entities.unit_comparator_by_y(*a, *b)
            });

            if self.unit_x_axis.len() == self.unit_y_axis.len() {
                dbg!(&self.unit_x_axis);
                dbg!(&self.unit_y_axis);
                panic!("number of units in x- and y-axis should match");
            }
        }

        for node in &mut self.nodes {
            // industry buy
            // industry production
            // industry sell
            // civilian consumption
        }
    }
}
