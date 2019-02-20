# Rough Ideas

brainstorm:
- blobing: politics, parties
- expansion: war, destruction
- growing: economy, logistics, corporation
- thinking: grid-base construction, adjacency, synergy
- complexity: character, family, relationship

setting:
- galactic-scale war, causing destruction to developed civs (different races biologically)
- civs scrambled multiple colony ships during the war; they happened to landed on the same planet where the game take place
- player control one character to do some open-end sandbox stuff
- add lore and stuff to cover up deficiencies for game mechanics later

game:
- people phase:
    - player spend character action points to perform actions that affect relationship, make decisions that may affect the world
    - number of available points depends on occupation, bonds, ???
- world phase:
    - change movement for player-controlled assets
    - acculate action points

goal (no real victory conditions, but designed for them):
- milestones
    - join the military
    - run successful corporations
    - form a faction, gain influence
    - gain support to form a nation, create unique identity
- end-game
    - world conquest (military victory)
    - form contacts with the old world (tech victory)

units:
- stats
    - life energy: decrease every turn, die upon reaching 0
    - hit points for body parts (arms, legs, body, head)
    - health: affect rate of change of life energy
        - physical health - affect how likely a character becomes sick (losing stats, life energy)
        - mental health - craziness cancels out actions, chance of accidents
    - satisfaction (affect change in health and rebelliousness)
        - consumption
        - exercise
        - fun
    - attributes
        - affect effectiveness
    - skills
        - affect effectiveness
    - action points (each point is worth 3 hrs, assume 8 hrs of sleep and 4 hrs of "living" time)
        - spent them to perform actions
        - 3 usable points
        - work shifts: 4, 8, 12 hours (1, 2, or 3 points)
        - use facilities
        - change relationships
        - pet project
        - overtime work dedication
- civilians not visible on the map, but on a list
- "deployed" units are visible on the map
    - act as leader of a squad
    - consumption relies on logistics units, if the unit isn't in a city
    - units follow group order, or leave the group (by splitting into a new group)

equipments:
- gun: short range
- sniper rifle: long-range, stealth skirmish
- bazooka: long range
- sword
    - very sharp sword that slice unarmed enemies
    - variant: energy sword - high-tech, point-blank weapon; high damage, piece through armor & cover
- uniform: unarmored, protective clothing
- exoskeleton: expensive, high-tech armor
    - force field: protect against bullets, medium movement, poor armor, medium dodge
    - lifter: carry powerful guns, low movement, high armor, low dodge
    - stealth: for assassination; glass cannon, high movement, high dodge
- pack animal: early-game commodity transportion; work best in difficult terrains; abundant in the early game, breeding doesn't scale well later
- truck: reliable transportation for highly developed infrastructure, become scalable in the mid game
- tank
    - reliable maneuverability in only high infrastructure
    - provide armor cover for infantry
    - spider tank: slower than and more expensive than normal tank, but highly mobile everywhere
- air support drones
    - shoot missiles; prioritize on air units; limit payloads
- bomber
    - drop mass destruction bombs
- dreadnought
    - large vehicle for transporting troops
    - act as a headquarter
    - variant: floating dreadnought
- engineering equipment: generic equipment for construction
- first-aid equipment: equipment for medics
- survival equipment: mandatory for fighting units; consumed when not in cities or transported by trucks

combat group
- forming groups:
    - leader decides unit composition and hire characters
    - characters up in the hierarchy aren't likely to be killed from combat
- role:
    - leader: one per group; attribute affect group
    - adjutant
        - 3 adjutants; leader counts as an extra one (i.e. four in total)
        - each one controls a squad
    - member
        - each one acts as a combat unit
        - each squad can have at most 6 people
        - adjutant of the squad can act as a member, at the risk of dying in combat
    - escortee: non-combatants who travel with the group; can leave freely
- squad
    - member slots for named characters
    - 1 optional slot for mobilized equipment (truck, tank)
        - dedicate 1 unit as driver
- 1 optional dreadnought slot
    - leader's squad to operate a dreadnought
- unit stats
    - readiness: 0-100%, break formation upon reaching 0%
    - firearm
        - the ability to use firearms
    - close quarter combat
        - the ability to perform melee combat (striking with weapons, dodging, blocking)
    - force manipulation
        - ???
    - armor
        - determined by armors or mobilized equipments
        - hitting flesh without armor = instant kill/limb loss
    - discovered
        - when an enemy discovers one of your units, all of enemies readiness will rise
- group stats
    - recon
        - affect view range, which makes units prepare against enemies sooner
        - groups that have higher recon gets a flat combat bonus against enemies
    - radar
        - detect stealth
    - stealth
        - stealth unit can perform backstabbing, which ignores defense skills
- combat
    - travel in group
    - when enemies in view range, start engaging them or escape
    - dreadnought (if available) sends out units
    - mobilized units sends out infantry
    - rain bullets
    - if wearing high-tech gears, engage in CQC
    - retreat, take prisoner, pack up, etc.; then resume travelling

relationship
- relation can be increase/decrease/spent through actions (maybe some are automatic)
- factors:
    - family
        - major bonus with first-degree relatives
        - minor bonus with second-degree relatives
    - friendship & comradeship
        - gain when engaging social activities/ fighting along side
        - spend relation 
    - romance
        - upgraded version of friendship, need player commands to form bonds
        - when romatic relation reach a threshold, there a chance of pregnancy when the 2 are in the same node

family
- one head per family; the head can designate an heir
- married family members who aren't the heir will split into a new family when
    - head is dead
    - bad relation with the new head
    - or head passes inheritance to the heir
    - own a house
- family influence
    - "credits" within a family
    - gaining points
        - adults automatically gain influence, children less so
        - being an heir gains more
        - be in a romantic relationship with another family's member (gaining points toward that family)
        - social projection - having high social credits gain rate
    - actions
        - inheritance - buy family assets with influence
        - gift - spend money to buy family influence
        - marriage proposal
        - adoption
        - establish independent family

construction projects
- simulation game-style, grid-base building, no road
    - extractor: extract raw materials based on category (mining, agriculture)
    - refinery: turn raw materials into intermediate materials
    - factory: produce finished goods
    - police

politics:
- people can become elites by spending influence
- all people can join political parties, but only elites can vote
- republic style government, election every 4 years
- city government
    - elite spent influence to mayor auction (candidates with the highest points win)
    - mayor decides public project
    - mayor vote for colonial governor
    - mayor can spend city funds to start construction projects
- colonial government
    - colonial governor vote for the president
    - change tax rate (colonial or provincial?)
- national goverment
    - actions
    - change tax rate
    - president:
        - set up 

world:
- a bunch of vertices that define cities with building radiuses, backed by a global grid for snapping buildings
- major vertices start with high population, high building radius
- minor vertices start with 0 population, low build radius, fixed list of raw resource available for extraction

economy

- production: each building take at most one resource to produce one intermediate/finished goods
- consumption: people, administrative consumption
- economy of scale: multiple buildings of the same type gives bonus production; adjacent building may also provide bonuses
- service: buildings that convert goods into people's well being, within their radius
- public buildings owned by the local government (city vertex)
- private buildings owned by family or corporations

military
- buildings owned by national government
- local government can raise militia force during emergency
- group of units band together, in formation


------

- land colony ship at a random node
- lore-wise all colonists wake up, pick up weapons and tools to become pioneers
- colony ship landing becomes the capital of a new nation
- most colonists are offscreen, but X colonists are elites (one of them is player-controlled)
- the elite with the highest charisma becomes the leader
- colony ship provides pioneer elites starting bonuses:
    - free first business
    - free equipment and manpower for a scout squad
- 4x gameplay
- victory conditions


-------

character <-> unit

character movement
