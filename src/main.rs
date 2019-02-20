extern crate regex;
#[macro_use]
extern crate enum_map;
extern crate nalgebra;
extern crate num;
extern crate ordered_float;
extern crate rand;
extern crate spade;
#[macro_use]
extern crate log;
extern crate getopts;
extern crate rand_hc;
extern crate simple_logger;

mod algorithm;
mod command;
mod game;
mod geometry;

use crate::command::character_enter_city;
use crate::command::create_farm;
use crate::command::create_warehouse;
use crate::command::deploy_character;
use crate::command::estimate_nearby_objects;
use crate::command::exit;
use crate::command::land_colony_ship;
use crate::command::list_characters;
use crate::command::list_nations;
use crate::command::list_nodes;
use crate::command::list_units;
use crate::command::move_unit;
use crate::command::step;
use crate::game::game::CITY_RADIUS;
use crate::game::game::CITY_RADIUS_SQUARED;
use crate::game::Commodity;
use crate::game::Game;
use crate::game::Node;
use getopts::Options;
use nalgebra::distance_squared;
use nalgebra::Point2;
use rand::Rng;
use rand::SeedableRng;
use rand_hc::Hc128Rng;
use regex::{Regex, RegexSet};
use std::env;
use std::io::{self, BufRead};

struct GameOptions {
    num_nodes: usize,
    dimension: (f32, f32),
    starting_population: usize,
    seed: Option<u64>,
}

impl Default for GameOptions {
    fn default() -> Self {
        Self {
            num_nodes: 10,
            dimension: (300., 300.),
            starting_population: 100,
            seed: None,
        }
    }
}

impl GameOptions {
    pub fn build(self) -> Game {
        let Self {
            num_nodes,
            dimension,
            starting_population,
            seed,
        } = self;

        let (width, height) = dimension;

        let mut rng = match seed {
            Some(state) => Hc128Rng::seed_from_u64(state),
            None => Hc128Rng::from_rng(rand::thread_rng()).unwrap(),
        };

        let nodes = {
            let mut try_counter = 0;
            let max_tries = 2 * num_nodes;

            let mut coor_candidates = Vec::with_capacity(100);
            let two_city_radius = CITY_RADIUS + CITY_RADIUS;
            while coor_candidates.len() < num_nodes {
                assert!(
                    try_counter < max_tries,
                    "exceed maximum number of tries in node generation"
                );

                try_counter += 1;

                let coor = {
                    let x = rng.gen_range(two_city_radius, width - two_city_radius);
                    let y = rng.gen_range(two_city_radius, height - two_city_radius);
                    Point2::new(x, y)
                };

                let is_overlapping = coor_candidates
                    .iter()
                    .any(|other_coor| distance_squared(other_coor, &coor) < CITY_RADIUS_SQUARED);

                if is_overlapping {
                    continue;
                }
                coor_candidates.push(coor);
            }

            coor_candidates
                .iter()
                .map(|&coor| Node {
                    coor,
                    population: starting_population,
                    institutions: Default::default(),
                    market: Default::default(),
                    resources: [
                        Commodity::Grain,
                        Commodity::Steel,
                        Commodity::Fruit,
                        Commodity::Meat,
                    ],
                    buildings: Default::default(),
                })
                .collect()
        };

        Game {
            nodes,
            rng,

            entities: Default::default(),
            nations: Default::default(),

            unit_destinations: Default::default(),
            building_grid: Default::default(),

            unit_nodes: Default::default(),
            unit_x_axis: Default::default(),
            unit_y_axis: Default::default(),
        }
    }

    pub fn set_num_nodes(self, num_nodes: usize) -> Self {
        Self { num_nodes, ..self }
    }

    pub fn set_seed(self, state: u64) -> Self {
        Self {
            seed: Some(state),
            ..self
        }
    }
}

type CommandDispatcher = dyn for<'r> std::ops::Fn(&'r mut Game, &Regex, &str) -> bool + 'static;
type RawCommands = std::vec::Vec<(std::string::String, &'static CommandDispatcher)>;

fn game_loop(mut g: Game) {
    let commands: RawCommands = {
        const FLOAT_REGEX: &str = r"-?\d+\.\d+|-?\d+";
        let coor_regex = format!(r"\(({}),({})\)", FLOAT_REGEX, FLOAT_REGEX);
        let estimate_nearby_objects_regex =
            format!("^rough nearby objects {} ({})$", coor_regex, FLOAT_REGEX);
        let create_farm_regex = format!("^create farm at {}$", coor_regex);
        let create_warehouse_regex = format!("^create warehouse at {}$", coor_regex);

        vec![
            ("^exit$".to_owned(), &exit),
            ("^step$".to_owned(), &step),
            ("^list nodes$".to_owned(), &list_nodes),
            (r"^move unit (\d+) to node (\d+)$".to_owned(), &move_unit),
            (
                estimate_nearby_objects_regex.to_owned(),
                &estimate_nearby_objects,
            ),
            (create_farm_regex, &create_farm),
            (create_warehouse_regex, &create_warehouse),
            ("^land colony ship$".to_owned(), &land_colony_ship),
            ("^list characters$".to_owned(), &list_characters),
            ("^list nations$".to_owned(), &list_nations),
            (r"^deploy character (\d+)$".to_owned(), &deploy_character),
            (
                r"^character (\d+) enter city$".to_owned(),
                &character_enter_city,
            ),
            (r"list units".to_owned(), &list_units),
        ]
    };

    let regexes: Vec<_> = commands
        .iter()
        .map(|(pattern, _)| Regex::new(pattern).unwrap())
        .collect();

    let patterns = commands.iter().map(|(pattern, _)| pattern);

    let set = RegexSet::new(patterns).unwrap();

    let mut match_list = Vec::with_capacity(commands.len());

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        match_list.clear();
        let line = line.unwrap();

        match_list.extend(set.matches(&line).into_iter());
        assert!(match_list.len() <= 1, "bug: ambiguous grammar for commands");

        if match_list.is_empty() {
            println!("invalid command: {}", line);
            continue;
        }

        let idx = match_list[0];
        let dispatcher = commands[idx].1;
        let regex = &regexes[idx];

        if dispatcher(&mut g, regex, &line) {
            break;
        }
    }
    println!("exited normally");
}

fn main() {
    fn do_work(inp: &str, out: Option<String>) {
        dbg!(format!("{}", 123232 + 324234));
        println!("input: {}", inp);
        match out {
            Some(x) => println!("output: {}", x),
            None => println!("No Output"),
        }
    }

    fn print_usage(program: &str, opts: Options) {
        let brief = format!("Usage: {} [options]", program);
        print!("{}", opts.usage(&brief));
    }

    simple_logger::init().unwrap();

    let args: Vec<String> = env::args().collect();

    println!("args: {:?}", args);

    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optopt("", "seed", "set seed for the game", "integer");
    opts.optflag("h", "help", "print this help menu");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string()),
    };
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    let options = GameOptions::default().set_num_nodes(15);

    /*
    let options = match matches.opt_str("seed") {
        Some(seed) => match seed.parse() {
            Ok(state) => options.set_seed(state),
            Err(err) => {
                panic!(err.to_string());
            }
        },
        None => options,
    };
    */

    let seed: u64 = rand::thread_rng().gen();
    println!("seed={}", seed);

    game_loop(options.set_seed(1026304851583305830).build());
}
