pub mod game;
pub mod update;

use crate::geometry::rectangle::Rectangle;
use enum_map::EnumMap;
use nalgebra::Point2;
use rand_hc::Hc128Rng;
use std::collections::{HashMap, HashSet};

pub trait MovableUnit {
    fn cal_max_steering_angle(&self) -> f32;
    fn cal_speed(&self) -> f32;
    // fn cal_next_coor(&self) -> NextCoor;
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum BuildingId {
    Warehouse(usize), // can have as many as corps can affort; regular maintenance

    /*
    - automatically built upon colonization, at the center of the city
    - Patrician-style market; act as money generator
    */
    Market(usize),

    /*
    - replaces market for the first colony
    - provide basic food, production, power generation
    */
    ColonyShipLanding(usize),

    AssemblyHall(usize), // one per city; unlock government options and private sector

    CorpHQ(usize), // at most one per corporation

    Farm(usize),       // produce crops, high economy of scale
    Mine(usize),       // produce metal
    Workshop(usize),   // high yield, low economy of scale
    Factory(usize),    // low yield, high economy of scale
    House(usize),      // low cost, low vacancy
    Skyscraper(usize), // high cost, high vacancy
}

#[derive(Default)]
pub struct Warehouse {
    storage: EnumMap<Commodity, u32>,
    demand_qtys: EnumMap<Commodity, u32>,
    import_prices: EnumMap<Commodity, f32>,
    sale_prices: EnumMap<Commodity, f32>,
}

pub struct Market {
    storage: EnumMap<Commodity, u32>,
}

pub struct Farm {
    output: u32,
}

pub struct Household {
    num_people: u32,
}

pub struct ColonyShipLanding {}

#[derive(Default)]
pub struct Buildings {
    farms: Vec<Farm>,
    warehouses: Vec<Warehouse>,
    households: Vec<Household>,
    landings: Vec<ColonyShipLanding>,
}

#[derive(Debug)]
pub enum CharacterMobility {
    Parked {
        node_idx: usize,
    },
    Unit {
        // hull size:
        //  - fixed upon construction
        //  - shipyards have different construction capabilities

        // limits
        // naval guns: perimeter - 1 = 2 * (length + width) - 1
        // interior:
        //      let dimension = (length * width * height)
        //      if dimension = 1 then number of interior slots = 0
        //      otherwise interior slots = dimension - number naval guns used

        // collision hits
        //  - coor represents the center of the unit
        //  - all units are rectangular
        //  - collision is based on the point-in-rectangle test
        body: Rectangle, // range search, collision detection(?)
        character_idx: usize,
        /*
        pub armor: u32,             // hp
        pub guns: EnumMap<Gun, u8>, // attack
        */
    },
}

#[derive(Debug)]
pub struct Character {
    pub children: HashSet<usize>,

    pub strength: u32,
    pub dexterity: u32,
    pub intelligence: u32,
    pub charisma: u32,

    pub mobility: CharacterMobility,
}

/*
Plan:
- business: a production/commerce building, either owned by a person or a corporation
- corporation: need a physical HQ, businesses operate as a whole; owned by

- business is a building owned by a corporation
- business prioritize trade with other businesses of the same owner corporations
- share holders are people only
- start a private corporation by building the first building
*/
struct Corporation {
    influence: u32,
    buildings: HashSet<usize>,
    ceo: usize, // character id,
    num_issued_shares: u32,
    shareholders: HashMap<usize, u32>, // character id, # shares
}

#[derive(Default)]
pub struct Nation {
    pub cities: HashSet<usize>,
}

struct Factions {
    cities: Vec<Node>,
    nations: Vec<Nation>,
    corporations: Vec<Corporation>,
}

trait Entity {
    fn get_x(&self) -> f32;
    fn get_y(&self) -> f32;
}

// create units - yes
// fire projectiles
// destr

#[derive(Default)]
pub struct Entities {
    // entities that are located on the map
    pub projectiles: Vec<Projectile>,
    pub buildings: Buildings,
    pub characters: Vec<Character>,
}

#[derive(Enum)]
pub enum Institution {
    Health,  // lower death rate
    Welfare, // increase birth rate
    Saftely,
    Law,
    Education,
    Research,
    Military,
}

#[derive(Default)]
pub struct InstitutionData {
    capacity: usize,    // effect capacity that the institution can provide
    actual: usize,      // actual cumulative value
    budget_rate: usize, // [0,1]
}

pub struct Node {
    // default radius
    pub market: EnumMap<Commodity, u32>,
    pub institutions: EnumMap<Institution, InstitutionData>,
    pub population: usize,
    pub coor: Point2<f32>,
    pub resources: [Commodity; 4],
    pub buildings: HashSet<BuildingId>,
}

#[derive(Clone, Copy, Hash, Eq, PartialEq, Debug)]
pub enum EntityId {
    Island(usize),
    Unit(usize),
    Character(usize),
}

// nodes produces goods
// nodes are immobile
// goods are needed for consumption
// nodes create objects
// objects move goods among nodes
// objects fight other objects and nodes
// when objects run out of hit points, they're destroyed
// when nodes run out of hit points, they are captured by attackers
// strategic

#[derive(Hash, Eq, PartialEq)]
enum Promotion {
    Striker,
    Skirmisher,
    Transporter,
    NumPromotions, // c-style count of the enum, not part of actual data
}

struct UnitStats {
    manpower: usize,
    attack: usize,
    defense: usize,
    armor: usize,
    payload: usize,
}

pub struct Game {
    pub rng: Hc128Rng,

    pub entities: Entities,

    pub nations: Vec<Nation>,

    pub unit_destinations: HashMap<usize, usize>, // character idx -> city index

    pub building_grid: HashMap<(u32, u32), BuildingId>,

    // nodes
    pub nodes: Vec<Node>,
    pub unit_nodes: HashMap<usize, usize>, // map unit idx to node idx

    pub unit_x_axis: Vec<usize>,
    pub unit_y_axis: Vec<usize>,
}

#[derive(Enum, Clone, Copy)]
pub enum Commodity {
    // primary resources
    Sand,
    Wood,   // for small buildings
    Cotton, // for clothings
    Hemp, // for clothings (more efficient than cotton) and drugs (if I am going to implement smuggling)
    Sugar,
    Cocoa,
    Diamond, // for jewelry
    Chemical,
    Rubber,

    // primary resources (strategic)
    Steel,     // weapons & tracked units
    Aluminum,  // aircraft
    Oil,       // basic(?) power
    Uranium,   // fission power
    Deuterium, // fusion power

    // primary resources (consumable)
    Grain,
    Fruit,
    Coffee,
    Spice,
    Tea,
    Gold, // consumed by banks to improve stability

    // secondary resources
    Silicon,  // from sand
    Glass,    // from sand
    Fat,      // from grain (feeding)
    Computer, // from alloy and silicon
    Plastic,  // from oil

    // final products
    Candy,     // from sugar
    Cake,      // from egg, sugar, milk
    Meat,      // from grain (feeding)
    Milk,      // from grain (feeding)
    Egg,       // from grain (feeding)
    Chocolate, // from sugar and cocoa
    Wine,      // from fruit
    Gadget,    // from computer and plastic
    Jewelry,   // from diamond
    Apparel,   // from hemp or cotton (separate buildings)
    Furniture, // from wood and hemp
    Vehicle,   // from steel and rubber
    Medicine,  // from chemical
}

#[derive(Enum)]
pub enum Gun {
    Gun,
    Artillery,
    EnergyBeam,
}

pub enum Projectile {
    Bullet {
        coor: Point2<f32>,
        destination: Point2<f32>,
        speed: f32,
    },
    Beam {
        fire_unit: EntityId,
        destination: Point2<f32>,
        speed: f32,
    },
}
