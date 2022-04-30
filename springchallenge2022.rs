#![allow(dead_code)]
#![allow(unused_variables)]

/**
CONTROL :
- It would be smarter to group mobs

unify naming (enemy vs other, ...)

1 DEF solo that would stay ahead and wind them
1 CLOSE that would farm mana
 */

use std::io;

const BASE_TARGET_DST: i32 = 5000;
const WIND_PUSH: f64 = 2200.0;
const WIND_DST: i32 = 1280;
const CONTROL_DST: i32 = 2200;
const SPELL_COST: i32 = 10;
const W: i32 = 17630;
const H: i32 = 9000;
const HW: i32 = W / 2;
const HH: i32 = H / 2;
const MVT_SPEED: i32 = 800;
const MONSTER_SPEED: i32 = 400;
const PLAYER_DMG_DST: i32 = 800;
const DST_TO_INFLICT_DMG_TO_BASE: i32 = 300;
const DMG: i32 = 2;
const NB_HEROS: i32 = 3;
const HALF_TERRAIN_DST: i32 = 9897; //(((W * W) + (H * H)) as f64).sqrt() / 2.0;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

#[derive(Eq, PartialEq, Clone, Debug)]
enum EntityType { Monster, MyHero, OtherHero }
#[derive(PartialEq, Clone, Debug)]
enum ThreatFor { Me, OtherBase, None }
#[derive(Clone, Debug, PartialEq)]
enum ActionType { Wind, Wait, Move, Control, Shield }
#[derive(Clone, Debug, PartialEq)]
enum HeroRole { Defense, Mid, Attack, NoRole }

#[derive(Clone, Debug)]
struct Action {
    a_type: ActionType,
    rank: i32,
    param: Pos,
    entity: i32,
    player: i32,
    player_pos: Pos,
    target_hp: i32,
    text: String,
    role: HeroRole,
}

impl Action {
    fn new_wind(value: i32, target: Pos, my_hero: &Entity, role: HeroRole) -> Action {
        Action {
            a_type: ActionType::Wind,
            rank: value,
            param: target,
            entity: 0,
            player: my_hero.id,
            player_pos: my_hero.pos.clone(),
            target_hp: 1,
            text: "ALL YOUR BASE ARE BELONG TO US".to_string(),
            role,
        }
    }
    fn new_shield(value: i32, target: Pos, target_entity: &Entity, my_hero: &Entity, role: HeroRole) -> Action {
        Action {
            a_type: ActionType::Shield,
            rank: value,
            param: target,
            entity: target_entity.id,
            player: my_hero.id,
            player_pos: my_hero.pos.clone(),
            target_hp: target_entity.hp,
            text: "WHAT YOU SAY".to_string(),
            role,
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
struct Pos {
    x: f64,
    y: f64,
}

impl Pos {
    fn dst(&self, pos: &Pos) -> i32 {
        let dst_x = self.x - pos.x;
        let dst_y = self.y - pos.y;
        (((dst_x * dst_x) + (dst_y * dst_y)) as f64).sqrt().abs() as i32
    }
    fn wind_me(&self, wind_origin: &Pos) -> Pos {
        let diff_x = self.x - wind_origin.x;
        let diff_y = self.y - wind_origin.y;
        let angle = diff_x.atan2(diff_y);

        let vector = Pos {
            x: WIND_PUSH * angle.sin(),
            y: WIND_PUSH * angle.cos(),
        };

        Pos { x: self.x + vector.x, y: self.y + vector.y }
    }
    fn add(&self, pos: &Pos) -> Pos {
        Pos {
            x: self.x + pos.x,
            y: self.y + pos.y,
        }
    }
    fn mv_center(&self, len: f64) -> Pos {
        let diff_x = HW as f64 - self.x;
        let diff_y = HH as f64 - self.y;
        let angle = diff_x.atan2(diff_y);

        let vector = Pos {
            x: len * angle.sin(),
            y: len * angle.cos(),
        };
        Pos { x: self.x + vector.x, y: self.y + vector.y }
    }
    fn go_there(&self, destination: &Pos, f: f64) -> Pos {
        Pos {
            x: (self.x * 1.0 - f) + (destination.x * f),
            y: (self.y * 1.0 - f) + (destination.y * f),
        }
    }
    fn center_symetry(&mut self) {
        self.x = W as f64 - self.x;
        self.y = H as f64 - self.y;
    }
}

#[derive(Clone, Debug)]
struct Entity {
    id: i32,
    entity_type: EntityType,
    pos: Pos,
    shield_life: i32,
    is_controlled: i32,
    hp: i32,
    velocity_x: i32,
    velocity_y: i32,
    near_base: i32,
    threat_for: ThreatFor,
    dst_from_base: i32,
    hero_role: HeroRole,
    threat_level: f64,
    threat_level_to_other: f64,
}
impl Entity {
    fn new() -> Entity {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();

        Entity {
            id: parse_input!(inputs[0], i32), // Unique identifier
            entity_type: match parse_input!(inputs[1], i32) {
                0 => EntityType::Monster,
                1 => EntityType::MyHero,
                _ => EntityType::OtherHero,
            },
            pos: Pos { x: parse_input!(inputs[2], i32) as f64, y: parse_input!(inputs[3], i32) as f64 },
            shield_life: parse_input!(inputs[4], i32), // Ignore for this league, Count down until shield spell fades
            is_controlled: parse_input!(inputs[5], i32), // Ignore for this league, Equals 1 when this entity is under a control spell
            hp: parse_input!(inputs[6], i32), // Remaining health of this monster
            velocity_x: parse_input!(inputs[7], i32), // Trajectory of this monster
            velocity_y: parse_input!(inputs[8], i32),
            near_base: parse_input!(inputs[9], i32), // 0=monster with no target yet, 1=monster targeting a base
            threat_for: match parse_input!(inputs[10], i32) {
                1 => ThreatFor::Me,
                2 => ThreatFor::OtherBase,
                _ => ThreatFor::None,
            },
            dst_from_base: 99999,
            hero_role: HeroRole::NoRole,
            threat_level: 1.0,
            threat_level_to_other: 1.0,
        }
    }
    fn next_pos(&self, turn: f64) -> Pos {
        Pos {
            x: self.pos.x + self.velocity_x as f64 * turn,
            y: self.pos.y + self.velocity_y as f64 * turn,
        }
    }
    fn compute_threat_level(&self, reference_base: &Pos) {

    }
}

#[derive(Debug)]
struct Stats {
    wind: i32,
    shield: i32,
    control: i32,
}
impl Stats {
    fn new() -> Stats { Stats { wind: 0, shield: 0, control: 0, } }
    fn process(&mut self, action: &Action) {
        match action.a_type {
            ActionType::Control => self.control += 1,
            ActionType::Wind => self.wind += 1,
            ActionType::Shield => self.shield += 1,
            _ => {}
        };
    }
    fn process_action(role: &HeroRole, def_stats: &mut Stats, att_stats: &mut Stats, action: &Action) {
        match role {
            HeroRole::Defense => def_stats.process(action),
            HeroRole::Attack => att_stats.process(action),
            _ => {}
        };
    }
}

struct Player {
    health: i32,
    mana: i32,
}

impl Player {
    fn new() -> Player {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        Player {
            health: parse_input!(inputs[0], i32),
            mana: parse_input!(inputs[1], i32),
        }
    }
}



fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split(" ").collect::<Vec<_>>();
    let my_base = Pos { x: parse_input!(inputs[0], i32) as f64, y: parse_input!(inputs[1], i32) as f64 };
    let other_base = Pos {
        x: if my_base.x < 10.0 {
            W as f64
        } else {
            0.0
        },
        y: if my_base.y < 10.0 {
            H as f64
        } else {
            0.0
        },
    };
    let mut other_base_edges = vec![
        Pos { x: 4700.0, y: 0.0 },
        Pos { x: 0.0,    y: 4700.0 },
    ];
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let heroes_per_player = parse_input!(input_line, i32); // Always 3
    eprintln!("My base {:?}. Other base {:?}", my_base, other_base);

    let mut def_positions = vec![
        Pos { x: 5500.0,    y: 2500.0 },
        Pos { x: 6000.0,    y: 5400.0 },
        Pos { x: 2500.0,    y: 5500.0 },

        // if I need to be closer
        Pos { x: 3500.0,    y: 1300.0 },
        Pos { x: 4000.0,    y: 3400.0 },
        Pos { x: 1300.0,    y: 3500.0 },
    ];
    let mut attack_positions = vec![
        Pos { x: 5000.0,    y: 3500.0 },

        Pos { x: 3000.0,    y: 5500.0 },
        Pos { x: 5500.0,    y: 3000.0 },
    ];
    if my_base.x > other_base.x {
        def_positions.iter_mut().for_each(|p| p.center_symetry());
    } else {
        attack_positions.iter_mut().for_each(|p| p.center_symetry());
        other_base_edges.iter_mut().for_each(|p| p.center_symetry());
    }

    let wind_push_45: f64 = WIND_PUSH * (1.0 as f64).sqrt();

    let wind_vectors = vec![
        Pos { x: WIND_PUSH,     y: 0.0 },           // RIGHT
        Pos { x: wind_push_45,  y: wind_push_45 },  // RIGHT UP
        Pos { x: wind_push_45,  y: -wind_push_45 }, // RIGHT DOWN

        Pos { x: 0.0, y: WIND_PUSH },               // UP
        Pos { x: 0.0, y: -WIND_PUSH },              // DOWN

        Pos { x: -WIND_PUSH,    y: 0.0 },           // LEFT
        Pos { x: -wind_push_45, y: wind_push_45 },  // LEFT UP
        Pos { x: -wind_push_45, y: -wind_push_45 }, // LEFT DOWN
    ];

    let mut def_stats = Stats::new();
    let mut att_stats = Stats::new();
    loop {
        let mut my_state = Player::new();
        let enemy_state = Player::new();

        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let entity_count: i32 = parse_input!(input_line, i32); // Amount of heros and monsters you can see
        let mut entities: Vec<Entity> = (0..entity_count).map(|_| Entity::new()).collect::<Vec<_>>();
        entities.iter_mut().for_each(|e| {
            e.dst_from_base = e.pos.dst(&my_base);
            let turns_to_hit = (e.dst_from_base - DST_TO_INFLICT_DMG_TO_BASE) as f64 / MONSTER_SPEED as f64;
            e.threat_level = e.hp as f64 / turns_to_hit;

            let turns_to_hit_other = (e.pos.dst(&other_base) - DST_TO_INFLICT_DMG_TO_BASE) as f64 / MONSTER_SPEED as f64;
            e.threat_level_to_other = e.hp as f64 / turns_to_hit_other;
        });

        let mut actions: Vec<Action> = compute_action(&my_base, &other_base, &mut my_state, &enemy_state, entities, &def_positions, &attack_positions, &wind_vectors, &other_base_edges);

        let mut selected_actions: Vec<Action> = Vec::new();
        while selected_actions.len() < 3 {
            actions.sort_by(|a, b| b.rank.partial_cmp(&a.rank).unwrap());
            let selected_action = actions[0].clone();
            update_on_selected_action(&mut actions, &selected_action, &my_base, &mut my_state, &wind_vectors);
            selected_actions.push(selected_action);
        }

        selected_actions.sort_by(|a, b| a.player.cmp(&b.player));
        selected_actions.iter().for_each(|action| {
            Stats::process_action(&action.role, &mut def_stats, &mut att_stats, &action);
            match action.a_type {
                ActionType::Move => println!("MOVE {} {} {:?}", action.param.x as i32, action.param.y as i32, action.role),
                ActionType::Wind => {
                    println!("SPELL WIND {} {} {:?}", action.param.x as i32, action.param.y as i32, action.role);
                    my_state.mana -= SPELL_COST;
                }
                ActionType::Control => println!("SPELL CONTROL {} {} {} {:?}", action.entity, action.param.x, action.param.y, action.role),
                ActionType::Shield => println!("SPELL SHIELD {} {:?}", action.entity, action.role),
                ActionType::Wait => println!("WAIT"),
            }
        });
        eprintln!("defense {:?}, attack {:?}", def_stats, att_stats)
    }
}

fn update_on_selected_action(actions: &mut Vec<Action>, selected_action: &Action, base: &Pos, my_state: &mut Player, wind_vectors: &Vec<Pos>) {
    actions.retain(|action| {
        action.player != selected_action.player
    });
    actions.iter_mut().for_each(|action| {
        // cannot cast another spell
        if selected_action.a_type == ActionType::Wind || selected_action.a_type == ActionType::Control || selected_action.a_type == ActionType::Shield {
            my_state.mana -= SPELL_COST;
            if my_state.mana < SPELL_COST && (action.a_type == ActionType::Wind || selected_action.a_type == ActionType::Control || selected_action.a_type == ActionType::Shield) {
                action.rank = -10000;
            }
        }
        // both wind, and maybe the same mobs.
        // NVM, wind vector will be ADDED, not avg
        //if selected_action.a_type == ActionType::Wind && action.a_type == ActionType::Wind && selected_action.player_pos.dst(&action.player_pos) < WIND_DST {
        //    action.rank /= 2;
        //}

        // I don't want to wind push someone that has already been targeted and is being acted upon. Or do I ?
        if action.a_type == ActionType::Wind && action.player_pos.dst(&selected_action.player_pos) < WIND_DST {
            action.rank /= 2;
        }
        // Control the same entity
        if selected_action.a_type == ActionType::Control && action.a_type == ActionType::Control && selected_action.entity == action.entity {
            action.rank = -10000;
        }
        if action.a_type == ActionType::Move && action.param.dst(&selected_action.param) < MVT_SPEED {
            let turn_to_kill = (action.target_hp + action.target_hp % DMG) / DMG;
            let turn_to_reach_base = action.param.dst(base);
            if turn_to_kill < turn_to_reach_base - 4 {
                action.rank /= 2;
            }
        }
    });
}

/**
WE ARE GOING TO HAVE TO ACT IF WE WANT TO LIVE IN A DIFFERENT WORLD
 */
fn compute_action(my_base: &Pos, other_base: &Pos, my_state: &mut Player, enemy_state: &Player, entities: Vec<Entity>, def_positions: &Vec<Pos>, attack_positions: &Vec<Pos>, wind_vectors: &Vec<Pos>, other_base_edges: &Vec<Pos>) -> Vec<Action> {
    let mut player_entities: Vec<Entity> = entities.iter().filter(|entity| entity.entity_type == EntityType::MyHero).cloned().collect();
    let enemy_entities: Vec<Entity> = entities.iter().filter(|entity| entity.entity_type == EntityType::OtherHero).cloned().collect();
    let monsters: Vec<Entity> = entities.iter().filter(|entity| entity.entity_type == EntityType::Monster).cloned().collect();

    // Assign HERO roles
    // find the hero closer to the enemy base and assign it to the attack role
    player_entities.sort_by(|h1, h2| h1.pos.dst(other_base).cmp(&h2.pos.dst(other_base)));
    player_entities[0].hero_role = HeroRole::Attack;
    player_entities[1].hero_role = HeroRole::Defense;
    player_entities[2].hero_role = HeroRole::Defense;

    let mut actions: Vec<Action> = Vec::new();

    player_entities.iter().for_each(|player| {
        match player.hero_role {
            HeroRole::Attack => compute_player_attack_action(&player, my_base, other_base, my_state, &player_entities, &enemy_entities, &monsters, def_positions, attack_positions, &mut actions, &wind_vectors, &other_base_edges),
            _ => compute_player_defensive_action(&player, my_base, other_base, my_state, &enemy_state, &player_entities, &enemy_entities, &monsters, def_positions, attack_positions, &mut actions, &wind_vectors, &other_base_edges),
        }
    });
    actions
}

/**
PREPARE TO ATTACK
 */
fn compute_player_attack_action(my_hero: &Entity, my_base: &Pos, other_base: &Pos, my_state: &mut Player, players: &Vec<Entity>, other_heroes: &Vec<Entity>, monsters: &Vec<Entity>, def_positions: &Vec<Pos>, attack_positions: &Vec<Pos>, actions: &mut Vec<Action>, wind_vectors: &Vec<Pos>, other_base_edges: &Vec<Pos>) {
    // ATTACK RESTING POSITION
    let mut resting_action = Action {
        a_type: ActionType::Move,
        rank: my_hero.pos.dst(&attack_positions[0]) / 4,
        param: attack_positions[0].clone(),
        entity: -1,
        player: my_hero.id,
        player_pos: my_hero.pos.clone(),
        target_hp: 1,
        text: "ATTACK".to_string(),
        role: HeroRole::Attack,
    };
    if my_hero.pos.dst(my_base) < my_hero.pos.dst(other_base) {
        resting_action.rank *= 2;
    }
    actions.push(resting_action);
    // ATTACK SPELL
    if my_state.mana > SPELL_COST {
        let defenders: Vec<Entity> = other_heroes.iter().filter(|e| e.pos.dst(other_base) < BASE_TARGET_DST).cloned().collect();
        // ATTACK CONTROL
        monsters.iter()
            .filter(|m| m.threat_for != ThreatFor::OtherBase && m.shield_life <= 0 && m.pos.dst(&my_hero.pos) < CONTROL_DST)
            .for_each(|m| {
                let mut value = 100 * (m.hp - defenders.len() as i32 * 10);
                // Make it go to the enemy base
                if m.threat_for == ThreatFor::Me {
                    value *= 2;
                }
                actions.push(Action {
                    a_type: ActionType::Control,
                    rank: value,
                    param: other_base.clone(),
                    entity: m.id,
                    player: my_hero.id,
                    player_pos: my_hero.pos.clone(),
                    target_hp: m.hp,
                    text: "ATTACK".to_string(),
                    role: HeroRole::Attack,
                });
            });
        // enemy_state.mana > SPELL_COST &&  => spells are applied to the targets and will only be effective on the next turn, after the next batch of commands. Does not protect from a spell from this same turn.
        if other_heroes.iter().any(|e| e.pos.dst(other_base) < BASE_TARGET_DST) {
            monsters.iter()
                .filter(|m| m.shield_life <= 0)
                .for_each(|m| {
                    let value = m.threat_level_to_other * 500.0;
                    if value > 1000.0 {
                        actions.push(Action::new_shield(value as i32, other_base.clone(), m, my_hero, HeroRole::Attack));
                    }
                });
        }

        // ATTACK WIND
        let wind_monsters: Vec<Entity> = monsters.iter().filter(|m| {
            m.pos.dst(&my_hero.pos) < WIND_DST && m.shield_life == 0
        }).cloned().collect();

        wind_vectors.iter().for_each(|wind_vector| {
            let mut wind_value = 0.0;
            // Try to make it closer to the enemy base
            // But really, they should land within base target distance
            wind_monsters.iter().for_each(|m| {
                let winded_dst_from_enemy_base = m.pos.add(wind_vector).dst(&other_base);
                wind_value += (m.pos.dst(&other_base) - winded_dst_from_enemy_base) as f64 * m.threat_level_to_other;
                wind_value /= 5.0;
                // or try to make go into the enemy base target distance for an otherwise non threatening monster
                if winded_dst_from_enemy_base < BASE_TARGET_DST {
                    wind_value *= 3.0;
                    if m.threat_for != ThreatFor::OtherBase {
                        wind_value *= 3.0;
                    }
                }
            });
            if my_state.mana < SPELL_COST * 3 {
                wind_value /= 4.0;
            }
            actions.push(Action::new_wind(wind_value as i32, my_hero.pos.add(wind_vector), my_hero, HeroRole::Attack))
        });
    }
    // ATTACK MOVE
    // target the nearest mob that is not a threat to the enemy base.
    monsters.iter().for_each(|m| {
        if m.threat_for != ThreatFor::OtherBase {
            actions.push(Action {
                a_type: ActionType::Move,
                rank: (600 * m.hp) - ((m.pos.dst(&my_hero.pos) - 600)),
                param: m.pos.clone(),
                entity: m.id,
                player: my_hero.id,
                player_pos: my_hero.pos.clone(),
                target_hp: m.hp,
                text: "ATTACK".to_string(),
                role: HeroRole::Attack,
            });
        }
    });
    // target an enemy that maybe be gaining some mana
    other_heroes.iter()
        .filter(|e| e.pos.dst(other_base) < e.pos.dst(my_base))
        .for_each(|e| {
            actions.push(Action {
                a_type: ActionType::Move,
                rank: 5000,
                param: e.pos.clone(),
                entity: e.id,
                player: my_hero.id,
                player_pos: my_hero.pos.clone(),
                target_hp: e.hp,
                text: "ALL YOUR BASE ARE BELONG TO US".to_string(),
                role: HeroRole::Attack,
            });
        })
}

/**
DEFENSE
 */
fn compute_player_defensive_action(my_hero: &Entity, base: &Pos, other_base: &Pos, my_state: &mut Player, enemy_state: &Player, my_heroes: &Vec<Entity>, other_heroes: &Vec<Entity>, monsters: &Vec<Entity>, def_positions: &Vec<Pos>, attack_positions: &Vec<Pos>, actions: &mut Vec<Action>, wind_vectors: &Vec<Pos>, other_base_edges: &Vec<Pos>) {
    // DEFENSE RESTING POSITION
    let rest_position =
        // if there is an enemy within base target dst and no friendlies
        if other_heroes.iter().any(|h| h.pos.dst(&base) < BASE_TARGET_DST) && my_heroes.iter().all(|h| h.pos.dst(&base) > BASE_TARGET_DST) {
            Pos { x:def_positions[(my_hero.id % NB_HEROS) as usize + 3].x, y: def_positions[(my_hero.id % NB_HEROS) as usize + 3].y }
        } else {
            Pos { x:def_positions[(my_hero.id % NB_HEROS) as usize].x, y: def_positions[(my_hero.id % NB_HEROS) as usize].y }
        };
    let resting_action = Action {
        a_type: ActionType::Move,
        rank: my_hero.pos.dst(&rest_position) / 4,
        param: rest_position,
        entity: -1,
        player: my_hero.id,
        player_pos: my_hero.pos.clone(),
        target_hp: 1,
        text: "DEFENSE".to_string(),
        role: HeroRole::Defense,
    };
    actions.push(resting_action);

    // DEFENSE ATTACKING
    monsters.iter().for_each(|m| {
        let mut value = match m.threat_for {
            ThreatFor::Me => (HALF_TERRAIN_DST - m.dst_from_base) * 2,
            ThreatFor::OtherBase => 0,
            ThreatFor::None => PLAYER_DMG_DST,
        };

        // It's not a direct threat, someone will already kill it, don't come. It ticks 2 hp per turn, so there is a margin. I will probably kill, and I am near it
        my_heroes.iter().for_each(|p| {
            if p.id != my_hero.id && p.pos.dst(&m.pos) < my_hero.pos.dst(&m.pos) {
                value -= (4000.0 / m.threat_level) as i32;
            }
        });

        let dst_score = (m.pos.dst(&my_hero.pos) as f64 / (MVT_SPEED as f64 / 80.0)) as i32;
        value -= dst_score;

        if my_state.mana < SPELL_COST * 4 {
            value *= 2;
        }
        actions.push(Action {
            a_type: ActionType::Move,
            rank: value,
            param: m.next_pos(0.5),
            entity: m.id,
            player: my_hero.id,
            player_pos: my_hero.pos.clone(),
            target_hp: m.hp,
            text: "DEFENSE".to_string(),
            role: HeroRole::Defense,
        });
    });

    let near_base_monsters: Vec<Entity> = monsters.iter().filter(|m| {
        m.dst_from_base < BASE_TARGET_DST + MONSTER_SPEED
    }).cloned().collect();
    // DEFENSE CASTING
    if my_state.mana > SPELL_COST {
        // DEFENSE SHIELD
        if my_hero.shield_life <= 1 && enemy_state.mana > SPELL_COST {
            let base_monsters: Vec<Entity> = monsters.iter().filter(|m| {
                m.dst_from_base < BASE_TARGET_DST
            }).cloned().collect();
            // can an enemy control me on the next turn ?
            if other_heroes.iter().any(|e| e.pos.dst(&my_hero.pos) < CONTROL_DST) && !base_monsters.is_empty() {
                actions.push(Action::new_shield(10000 * base_monsters.len() as i32, my_hero.pos.clone(), my_hero, my_hero, HeroRole::Defense));
            }
        }
        // DEFENSE CONTROL
        monsters.iter().filter(|m| {
            m.pos.dst(&my_hero.pos) < CONTROL_DST && m.shield_life == 0 && m.threat_for != ThreatFor::OtherBase
        }).for_each(|m| {
            // AWAY FROM BASE let's assume it's going to hit.
            let mut value = 250.0 * m.threat_level;
            // Here we want it to go away
            if m.dst_from_base < (DST_TO_INFLICT_DMG_TO_BASE + (MONSTER_SPEED as f64 * (m.hp as f64 / 2.0)) as i32) {
                value *= 15.0;
            }
            let target_pos = other_base.clone();
            actions.push(Action {
                a_type: ActionType::Control,
                rank: value as i32,
                param: target_pos,
                entity: m.id,
                player: my_hero.id,
                player_pos: my_hero.pos.clone(),
                target_hp: m.hp,
                text: "DEFENSE".to_string(),
                role: HeroRole::Defense,
            });
        });
        // push an enemy to my base
        /*
        enemies.iter().filter(|e| {
            e.pos.dst(&player.pos) < CONTROL_DST && e.shield_life == 0
        }).for_each(|e| {
            actions.push(Action {
                a_type: ActionType::Control,
                rank: 1000 * near_base_monsters.len() as i32,
                param: base.clone(),
                entity: e.id,
                player: player.id,
                player_pos: player.pos.clone(),
                target_hp: 1,
            });
        });
        */

        // DEFENSE WIND
        let wind_monsters: Vec<Entity> = monsters.iter().filter(|m| {
            m.pos.dst(&my_hero.pos) < WIND_DST && m.shield_life == 0
        }).cloned().collect();
        wind_vectors.iter().for_each(|wind_vector| {
            let mut wind_value = 0;
            wind_monsters.iter().for_each(|m| {
                let winded_base_dst = m.pos.add(wind_vector).dst(&base);
                wind_value += (winded_base_dst - m.dst_from_base) * (m.hp as f64) as i32;
                // it loses its target which is great because it will pick a new orientation at random
                if winded_base_dst > BASE_TARGET_DST && m.dst_from_base < BASE_TARGET_DST {
                    wind_value *= 4;
                }
                if m.threat_for != ThreatFor::Me {
                    wind_value /= 8;
                }
                if m.dst_from_base > BASE_TARGET_DST {
                    wind_value /= 8;
                }
                wind_value /= 5;
            });
            actions.push(Action::new_wind(wind_value, my_hero.pos.add(wind_vector), my_hero, HeroRole::Defense))
        });
    }
}


#[cfg(test)]
mod test {
    use std::f64::consts::FRAC_1_SQRT_2;
    use super::*;

    #[test]
    fn wind_pos_test() {
        let wind_pos = Pos { x: 5000.0, y: 5000.0 };
        assert_eq!(wind_pos.wind_me(&Pos { x: 5000.0 - 1.0, y: 5000.0 }), Pos { x: 5000.0 + WIND_PUSH, y: 5000.0 });
        assert_eq!(wind_pos.wind_me(&Pos { x: 5000.0 + 1.0, y: 5000.0 }), Pos { x: 5000.0 - WIND_PUSH, y: 5000.0 });
        assert_eq!(wind_pos.wind_me(&Pos { x: 5000.0, y: 5000.0 - 1.0 }), Pos { x: 5000.0, y: 5000.0 + WIND_PUSH });
        assert_eq!(wind_pos.wind_me(&Pos { x: 5000.0, y: 5000.0 + 1.0 }), Pos { x: 5000.0, y: 5000.0 - WIND_PUSH });
        assert_eq!(wind_pos.wind_me(&Pos { x: 6000.0, y: 6000.0 }), Pos { x: 5000.0 - WIND_PUSH * FRAC_1_SQRT_2, y: 5000.0 - WIND_PUSH * FRAC_1_SQRT_2 });
    }

    #[test]
    fn mv_center_test() {
        assert_eq!(Pos { x: 0.0, y: 0.0 }.mv_center(1000.0), Pos { x: 890.6575017425195, y: 454.67484490542694 });
        assert_eq!(Pos { x: W as f64, y: HH as f64 }.mv_center(1000.0), Pos { x: W as f64 - 1000.0, y: HH as f64 });
        assert_eq!(
            Pos { x: W as f64, y: H as f64 }.mv_center(1000.0),
            Pos { x: 16922.893218813453, y: 8292.893218813453 });
    }

    #[test]
    fn dst_test() {
        assert_eq!(Pos { x: 0.0, y: 0.0 }.dst(&Pos { x: 0.0, y: 0.0 }), 0);
        assert_eq!(Pos { x: 100.0, y: 0.0 }.dst(&Pos { x: 0.0, y: 0.0 }), 100);
        assert_eq!(Pos { x: 0.0, y: 100.0 }.dst(&Pos { x: 0.0, y: 0.0 }), 100);
        assert_eq!(Pos { x: 0.0, y: 0.0 }.dst(&Pos { x: 100.0, y: 0.0 }), 100);
        assert_eq!(Pos { x: 0.0, y: 0.0 }.dst(&Pos { x: 0.0, y: 100.0 }), 100);
        assert_eq!(Pos { x: 100.0, y: 100.0 }.dst(&Pos { x: 0.0, y: 0.0 }), 141);
    }
}