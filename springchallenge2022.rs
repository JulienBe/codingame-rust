#![allow(dead_code)]
#![allow(unused_variables)]

/**
CONTROL :
- It would be smarter to group mobs
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
const MONSTER_DMG_DST: i32 = 300;
const DMG: i32 = 2;
const NB_HEROS: i32 = 3;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

#[derive(Eq, PartialEq, Clone)]
enum EntityType { Monster, MyHero, OtherHero }
#[derive(PartialEq, Clone, Debug)]
enum ThreatFor { Me, OtherBase, None }
#[derive(Clone, Debug, PartialEq)]
enum ActionType { Wind, Wait, Move, Control }
#[derive(Clone, Debug, PartialEq)]
enum HeroRole { DEFENSE, MID, ATTACK, NO_ROLE }

#[derive(Clone, Debug)]
struct Action {
    a_type: ActionType,
    rank: i32,
    param: Pos,
    entity: i32,
    player: i32,
    player_pos: Pos,
    target_hp: i32,
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
    fn wind_vector(&self, wind_origin: &Pos) -> Pos {
        let diff_x = self.x - wind_origin.x;
        let diff_y = self.y - wind_origin.y;
        let angle = diff_x.atan2(diff_y);
        Pos {
            x: WIND_PUSH * angle.sin(),
            y: WIND_PUSH * angle.cos(),
        }
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
}

#[derive(Clone)]
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
}

impl Entity {
    fn next_pos(&self, turn: f64) -> Pos {
        Pos {
            x: self.pos.x + self.velocity_x as f64 * turn,
            y: self.pos.y + self.velocity_y as f64 * turn,
        }
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
            hero_role: HeroRole::NO_ROLE,
        }
    }
}

fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split(" ").collect::<Vec<_>>();
    let base = Pos { x: parse_input!(inputs[0], i32) as f64, y: parse_input!(inputs[0], i32) as f64 };
    let enemy_base = Pos {
        x: if base.x < 1.0 {
            W as f64
        } else {
            0.0
        },
        y: if base.y < 1.0 {
            H as f64
        } else {
            0.0
        },
    };
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let heroes_per_player = parse_input!(input_line, i32); // Always 3

    let mut def_positions = vec![
        Pos { x: 6000.0,    y: 1200.0 },
        Pos { x: 6000.0,    y: 5400.0 },
        Pos { x: 1800.0,    y: 5500.0 },

        Pos { x: 3400.0,    y: 5300.0 },
        Pos { x: 5500.0,    y: 3000.0 },

        Pos { x: 5000.0,    y: 5000.0 },
    ];
    let mut attack_positions = vec![
        Pos { x: 5000.0,    y: 5000.0 },

        Pos { x: 3000.0,    y: 5500.0 },
        Pos { x: 5500.0,    y: 3000.0 },
    ];
    if base.x > 1.0 {
        def_positions.iter_mut().for_each(|o| {
            o.x = W as f64 - o.x;
        });
    } else {
        attack_positions.iter_mut().for_each(|o| {
            o.x = W as f64 - o.x;
        });
    }
    if base.y > 1.0 {
        def_positions.iter_mut().for_each(|o| {
            o.y = H as f64 - o.y;
        });
    } else {
        attack_positions.iter_mut().for_each(|o| {
            o.x = W as f64 - o.x;
        });
    }

    loop {
        let mut my_state = Player::new();
        let enemy_state = Player::new();

        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let entity_count: i32 = parse_input!(input_line, i32); // Amount of heros and monsters you can see
        let entities: Vec<Entity> = (0..entity_count).map(|_| Entity::new()).collect::<Vec<_>>();

        let mut actions: Vec<Action> = compute_action(&base, &enemy_base, &mut my_state, entities, &def_positions, &attack_positions);
        //actions.sort_by(|a, b| b.rank.partial_cmp(&a.rank).unwrap());
        //actions.iter().for_each(|a| eprintln!("action: {:?}", a));

        let mut selected_actions: Vec<Action> = Vec::new();
        while selected_actions.len() < 3 {
            actions.sort_by(|a, b| b.rank.partial_cmp(&a.rank).unwrap());
            let selected_action = actions[0].clone();
            update_on_selected_action(&mut actions, &selected_action, &base);
            selected_actions.push(selected_action);
        }

        selected_actions.sort_by(|a, b| a.player.cmp(&b.player));
        selected_actions.iter().for_each(|action| {
            match action.a_type {
                ActionType::Move => println!("MOVE {} {}", action.param.x as i32, action.param.y as i32),
                ActionType::Wind => {
                    println!("SPELL WIND {} {}", action.param.x as i32, action.param.y as i32);
                    my_state.mana -= SPELL_COST;
                }
                ActionType::Control => println!("SPELL CONTROL {} {} {}", action.entity, action.param.x, action.param.y),
                ActionType::Wait => println!("WAIT"),
            }
        });
    }
}

fn update_on_selected_action(actions: &mut Vec<Action>, selected_action: &Action, base: &Pos) {
    actions.retain(|action| {
        action.player != selected_action.player
    });
    actions.iter_mut().for_each(|action| {
        // I don't want to wind push someone that has already been targeted and is being acted upon. Or do I ?
        if action.a_type == ActionType::Wind && action.player_pos.dst(&selected_action.player_pos) < WIND_DST {
            action.rank /= 2;
        }
        // Control the same entity
        if selected_action.a_type == ActionType::Control && action.a_type == ActionType::Control && selected_action.entity == action.entity {
            action.rank = 0;
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

fn compute_action(base: &Pos, enemy_base: &Pos, my_state: &mut Player, entities: Vec<Entity>, def_positions: &Vec<Pos>, attack_positions: &Vec<Pos>) -> Vec<Action> {
    let mut player_entities: Vec<Entity> = entities.iter().filter(|entity| entity.entity_type == EntityType::MyHero).cloned().collect();
    let enemy_entities: Vec<Entity> = entities.iter().filter(|entity| entity.entity_type == EntityType::OtherHero).cloned().collect();
    let mut monsters: Vec<Entity> = entities.iter().filter(|entity| entity.entity_type == EntityType::Monster).cloned().collect();
    monsters.iter_mut().for_each(|m| m.dst_from_base = m.pos.dst(base));

    // Assign HERO roles
    // find the hero closer to the enemy base and assign it to the attack role
    player_entities.sort_by(|h1, h2| h1.pos.dst(enemy_base).cmp(&h2.pos.dst(enemy_base)));
    player_entities[0].hero_role = HeroRole::ATTACK;
    player_entities[1].hero_role = HeroRole::DEFENSE;
    player_entities[2].hero_role = HeroRole::DEFENSE;

    let mut actions: Vec<Action> = Vec::new();

    player_entities.iter().for_each(|player| {
        // RESTING POSITION
        let rest_position = Pos { x:def_positions[(player.id % NB_HEROS) as usize].x, y: def_positions[(player.id % NB_HEROS) as usize].y };
        let resting_action = Action {
            a_type: ActionType::Move,
            rank: player.pos.dst(&rest_position) / 3,
            param: rest_position,
            entity: -1,
            player: player.id,
            player_pos: player.pos.clone(),
            target_hp: 1,
        };
        actions.push(resting_action);

        // ATTACKING
        monsters.iter_mut().for_each(|m| {
            let mut base_value = match m.threat_for {
                ThreatFor::Me => W - m.dst_from_base,
                ThreatFor::OtherBase => -20000,
                ThreatFor::None => PLAYER_DMG_DST,
            };
            base_value -= m.pos.dst(&player.pos) / (MVT_SPEED / 2);
            actions.push(Action {
                a_type: ActionType::Move,
                rank: base_value,
                param: m.next_pos(0.5),
                entity: m.id,
                player: player.id,
                player_pos: player.pos.clone(),
                target_hp: m.hp
            });
        });

        // CASTING
        if my_state.mana > SPELL_COST {
            // CONTROL
            let control_monsters: Vec<Entity> = monsters.iter().filter(|m| {
                m.pos.dst(&player.pos) < CONTROL_DST
            }).cloned().collect();

            control_monsters.iter().for_each(|m| {
                let next_pos = m.next_pos(2.0);
                let next_dst_from_base = next_pos.dst(base);
                // AWAY FROM BASE let's assume it's going to hit.
                if next_dst_from_base < MONSTER_DMG_DST {
                    actions.push(Action {
                        a_type: ActionType::Control,
                        rank: 20000,
                        param: m.next_pos(-2.0),
                        entity: m.id,
                        player: player.id,
                        player_pos: player.pos.clone(),
                        target_hp: m.hp,
                    });
                }
                if m.threat_for != ThreatFor::OtherBase {
                    actions.push(Action {
                        a_type: ActionType::Control,
                        rank: 1000 * (m.hp - 12),
                        param: enemy_base.clone(),
                        entity: m.id,
                        player: player.id,
                        player_pos: player.pos.clone(),
                        target_hp: m.hp,
                    });
                }
            });
            enemy_entities.iter().filter(|e| {
                e.pos.dst(&player.pos) < CONTROL_DST
            }).for_each(|e| {
                actions.push(Action {
                    a_type: ActionType::Control,
                    rank: 10000,
                    param: base.clone(),
                    entity: e.id,
                    player: player.id,
                    player_pos: player.pos.clone(),
                    target_hp: 1,
                });
            });
            // WIND
            let wind_monsters: Vec<Entity> = monsters.iter().filter(|m| {
                m.pos.dst(&player.pos) < WIND_DST
            }).cloned().collect();
            let targets_i_consider: Vec<Pos> = wind_monsters.iter().filter(|m| {
                let winded_pos = m.pos.wind_me(&player.pos);
                let winded_base_dst = m.pos.dst(&base);
                m.dst_from_base < winded_base_dst
            }).map(|m| {
                m.pos.wind_vector(&m.pos)
            }).collect();
            targets_i_consider.iter().for_each(|wind_vector| {
                let mut wind_value = 0;
                wind_monsters.iter().for_each(|m| {
                    let winded_pos = m.pos.add(wind_vector);
                    let winded_base_dst = m.pos.dst(&base);
                    wind_value += (winded_base_dst - m.dst_from_base) * 3;
                    // it loses its target which is great because it will pick a new orientation at random
                    if winded_base_dst > BASE_TARGET_DST && m.dst_from_base < BASE_TARGET_DST {
                        wind_value += 5000;
                    }
                });
                actions.push(Action {
                    a_type: ActionType::Wind,
                    rank: wind_value,
                    param: Pos { x: wind_vector.x, y: wind_vector.y },
                    entity: 0,
                    player: player.id,
                    player_pos: player.pos.clone(),
                    target_hp: 1,
                })
            });
        }
    });
    actions
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