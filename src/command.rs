use crate::game::CharacterMobility;
use crate::game::EntityId;
use crate::game::Game;
use crate::game::MovableUnit;
use crate::geometry::rectangle::Rectangle;
use nalgebra::Point2;
use rand::Rng;
use regex::Regex;

pub fn exit(_: &mut Game, _: &Regex, _: &str) -> bool {
    true
}

pub fn step(g: &mut Game, _: &Regex, _: &str) -> bool {
    g.update_state();
    debug!("step");
    false
}

pub fn list_nodes(g: &mut Game, _: &Regex, _: &str) -> bool {
    println!("{:>16} {:>16}", "index", "coor");
    for (i, node) in g.nodes.iter().enumerate() {
        println!(
            "{:>16} {:>16}",
            i,
            format!("({},{})", node.coor[0] as u32, node.coor[1] as u32)
        );
    }
    false
}

pub fn list_units(g: &mut Game, _: &Regex, _: &str) -> bool {
    println!(
        "{:>16} {:>32} {:>16} {:>32}",
        "char id", "coor", "speed", "goal"
    );
    for (character_idx, character) in g.entities.characters.iter().enumerate() {
        if let CharacterMobility::Unit { body, .. } = character.mobility {
            let coor = body.center;
            let unit_x = coor.x;
            let unit_y = coor.y;
            println!(
                "{:>16} {:>32} {:>16} {:>32}",
                character_idx,
                format!("({:.2},{:.2})", unit_x, unit_y),
                character.cal_speed(),
                match g.unit_destinations.get(&character_idx) {
                    Some(destination) => {
                        //
                        let coor = g.nodes[*destination].coor;
                        format!(
                            "{} - {:>20}",
                            destination,
                            format!("({:.2},{:.2})", coor.x, coor.y)
                        )
                    }
                    None => "-".to_string(),
                }
            );
        } else {
            //
        }
    }
    false
}

pub fn move_unit(g: &mut Game, regex: &Regex, line: &str) -> bool {
    let group = regex.captures(line).unwrap();

    let character_idx = group[1].parse().unwrap();
    let node_idx = group[2].parse().unwrap();

    println!(
        "trying to move character {} to node {}",
        character_idx, node_idx
    );
    g.set_unit_destination(character_idx, node_idx);

    false
}

pub fn deploy_character(g: &mut Game, regex: &Regex, line: &str) -> bool {
    let group = regex.captures(line).unwrap();

    let character_idx: usize = group[1].parse().unwrap();

    if character_idx >= g.entities.characters.len() {
        println!("invalid character id");
        return false;
    }

    let character = &mut g.entities.characters[character_idx];

    if let CharacterMobility::Parked { node_idx } = character.mobility {
        let coor = g.nodes[node_idx].coor;
        let unit = CharacterMobility::Unit {
            body: Rectangle {
                length: 1.,
                width: 1.,
                center: coor,
                angle: 0.,
            },
            character_idx,
        };

        character.mobility = unit;

        g.unit_x_axis.push(character_idx);
        g.unit_y_axis.push(character_idx);
    } else {
        debug!("character isn't parked in a city");
        return false;
    }

    false
}

pub fn estimate_nearby_objects(g: &mut Game, regex: &Regex, line: &str) -> bool {
    let group = regex.captures(&line).unwrap();

    let x = group[1].parse().unwrap();
    let y = group[2].parse().unwrap();
    let r = group[3].parse().unwrap();

    for idx in g.estimate_nearby_units(Point2::new(x, y), r) {
        let coor = g.get_unit_coor(idx);
        println!("{:?} - {}", idx, coor);
    }
    false
}

pub fn character_enter_city(g: &mut Game, regex: &Regex, line: &str) -> bool {
    // TODO

    let group = regex.captures(line).unwrap();

    let character_idx: usize = group[1].parse().unwrap();

    if character_idx >= g.entities.characters.len() {
        println!("invalid character id");
        return false;
    }

    let character = &mut g.entities.characters[character_idx];

    if let CharacterMobility::Parked { node_idx } = character.mobility {
        let coor = g.nodes[node_idx].coor;
        let unit = CharacterMobility::Unit {
            body: Rectangle {
                length: 1.,
                width: 1.,
                center: coor,
                angle: 0.,
            },
            character_idx,
        };

        character.mobility = unit;

        g.unit_x_axis.push(character_idx);
        g.unit_y_axis.push(character_idx);
    } else {
        debug!("character isn't parked in a city");
        return false;
    }

    false
}

pub fn create_farm(g: &mut Game, regex: &Regex, line: &str) -> bool {
    let group = regex.captures(&line).unwrap();

    let x = group[1].parse().unwrap();
    let y = group[2].parse().unwrap();

    if let Some(id) = g.create_farm([x, y]) {
        println!("created {:?} at ({},{})", id, x, y);
    } else {
        println!("cannot create farm at ({},{})", x, y);
    }
    false
}

pub fn create_warehouse(g: &mut Game, regex: &Regex, line: &str) -> bool {
    let group = regex.captures(&line).unwrap();

    let x = group[1].parse().unwrap();
    let y = group[2].parse().unwrap();

    if let Some(id) = g.create_warehouse([x, y]) {
        println!("created {:?} at ({},{})", id, x, y);
    } else {
        println!("cannot create warehouse at ({},{})", x, y);
    }
    false
}

pub fn land_colony_ship(g: &mut Game, _: &Regex, _: &str) -> bool {
    let id = g.colonize();
    assert!(id.is_some());
    println!("colony landing building id: {:?}", id);

    false
}

pub fn list_characters(g: &mut Game, _: &Regex, _: &str) -> bool {
    println!(
        "{:>16} {:>16} {:>16} {:>16} {:>16} {:>16} {:>30}",
        "id", "strength", "dexterity", "intelligence", "charisma", "# childs", "mobility"
    );

    for (character_idx, c) in g.entities.characters.iter().enumerate() {
        let num_children = c.children.len();

        let mobility = match c.mobility {
            CharacterMobility::Unit { body, .. } => format!("AT {}", body.center), // TODO
            CharacterMobility::Parked { node_idx } => format!("PARKED {}", node_idx),
        };

        //
        println!(
            "{:>16} {:>16} {:>16} {:>16} {:>16} {:>16} {:>30}",
            character_idx,
            c.strength,
            c.dexterity,
            c.intelligence,
            c.charisma,
            num_children,
            mobility,
        );
    }

    false
}

pub fn list_nations(g: &mut Game, _: &Regex, _: &str) -> bool {
    println!("{:>16} {:>16}", "index", "# cities");

    for (idx, nation) in g.nations.iter().enumerate() {
        let num_cities = nation.cities.len();
        //
        println!("{:>16} {:>16}", idx, num_cities);
    }

    false
}
