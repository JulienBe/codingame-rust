/*

Earn more rupees than your opponent!

The game takes place in a **potion shop**, in which two twin-sister witches are trying to prove they are the better potion brewer.

They have set up a contest: make more rupees selling potions than your sister.

However, the witch's hut in witch they set up shop is quite small, so they must share the same workspace, and deal with the same **client orders**.

# Rules

Each player controls a witch, each witch has access to their own inventory of potion ingredients and a list of spells they have learnt.

These spells can be used to turn a certain set of ingredients into another.

Each client order is a list of ingredients required to brew a potion and earn some rupees.

#  Ingredients

There are 4 tiers of ingredient, indexed from 0 to 3.

A witch's inventory can contain up to 10 ingredients.

Each witch starts with 3 tier-0 ingredients in their inventory.

The inventory is represented by inv: 4 numbers each representing the amount of each ingredient tier.

inv = [3,1,0,0]

# Action overview

Each round, you can perform one of the following actions:

- Cast one of your spells.
- Rest to refresh all previously cast spells.
- Brew a potion to score points.

You may also opt to skip a turn with the WAIT command.

#  Casting Spells

Spells have a delta: 4 numbers, one for each ingredient tier.

Positive numbers represent the amount of ingredients that are produced by the recipe.

Negative numbers represent the amount of ingredients that are consumed by the recipe.

For instance, a spell marked -1,1,0,0 means it can turn one tier-0 ingredient into a tier-1 ingredient.

Once you have cast a spell, it becomes exhausted. You may not cast exhausted spells.

Some spells do not consume ingredients, they simply produce new ingredients.

Each player spell has a unique id and can be cast with the CAST id command.


# Resting

Resting lets you channel your magic, rendering all exhausted spells available again for casting.

You can order your witch to rest with the REST command.

#  Brewing

Client orders have a delta: 4 numbers, one for each ingredient tier.

Negative numbers represent the amount of ingredients that are consumed by the recipe.

Therefore, the numbers are never positive because they represent a loss of ingredients from your inventory.

For instance, a client order with delta = -2, -1, 0, 0 means you have to consume 2 tier-0 ingredients and 1 tier-2 ingredients from your inventory in order to brew the potion.

The selling price of the client order is the amount of rupees will you earn by completing it.

The client orders are queued up from left to right. Only five clients can fit inside the hut so a maximum of 5 orders will be available every turn.

If both witches brew the same potion, they both earn its price in rupees.

At the start of each new turn, new orders are queued up to fill the missing spaces.

Each order has a unique id and can be undertaken with the BREW id command.

You may also opt to skip a turn with the WAIT command.

# Game end

The game ends once at least one witch has brewed 3 potions.

The game stops automatically after 100 rounds.

Players gain 1 rupee for each tier-1 ingredient or higher in their inventory.

# Victory Conditions

The winner is the player with the most rupees.

#  Game Protocol
## Input for One Game Turn
### Line 1

one integer actionCount for the number of all available client orders.

Next actionCount lines: 11 space-separated values to describe a game action.

- actionId: the id of this action
- actionType: a string
- - BREW for a potion recipe
- delta0, delta1, delta2, delta3: the four numbers describing the consumption/producion of ingredients for each tier.
- price the amount of rupees this will win you if this is a potion recipe, 0 otherwise.
- tomeIndex: ignore for this league.
- taxCount: ignore for this league.
- castable: ignore for this league.
- repeatable: ignore for this league.

### Next 2 lines

5 integers to describe each player, your data is always first:

- inv0 for the amount of tier-0 ingredients in their inventory
- inv1 for the amount of tier-1 ingredients in their inventory
- inv2 for the amount of tier-2 ingredients in their inventory
- inv3 for the amount of tier-3 ingredients in their inventory
- score for the amount of rupees earned so far

##
Output
A single line with your command:

- BREW id: your witch attempts to brew the potion with the given id.
- WAIT: your witch does nothing.

# Constraints
- 0 < actionCount ≤ 100
- 6 ≤ price ≤ 23
- Response time per turn ≤ 50ms
- Response time for the first turn ≤ 1000ms
 */

use std::{io, fmt};
use crate::Action::{BREW, WAIT, REST, CAST};
use std::fmt::{Display, Formatter};

macro_rules! parse {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

const INGREDIENTS_TIER: usize = 4;
const MAX_INGREDIENTS: usize = 10;

#[derive(Debug)]
enum Action {
    BREW,
    REST,
    CAST,
    WAIT,
}
#[derive(Debug)]
struct Command {
    action: Action,
    id: i32,
    score: i32,
}
#[derive(Debug)]
struct Delta {
    val: [i32; 4],
}
#[derive(Debug)]
struct Player {
    ingredients: Delta,
    score: i32,
}
#[derive(Debug)]
struct Spell {
    id: i32,
    delta: Delta,
    exhausted: bool,
}
#[derive(Debug)]
struct Potion {
    id: i32,
    price: i32,
    delta: Delta,
}

fn main() {
    loop {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let action_count = parse!(input_line, i32); // the number of spells and recipes in play

        let mut potions: Vec<Potion> = vec![];
        let mut spells: Vec<Spell> = vec![];
        for i in 0..action_count as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let inputs = input_line.split(" ").collect::<Vec<_>>();
            let action_id = parse!(inputs[0], i32); // the unique ID of this spell or recipe
            let action_type = Action::convert(inputs[1].trim());
            let delta = Delta { val: [parse!(inputs[2], i32), parse!(inputs[3], i32), parse!(inputs[4], i32), parse!(inputs[5], i32)] };
            let price = parse!(inputs[6], i32); // the price in rupees if this is a potion
            let tome_index = parse!(inputs[7], i32); // in the first two leagues: always 0; later: the index in the tome if this is a tome spell, equal to the read-ahead tax
            let tax_count = parse!(inputs[8], i32); // in the first two leagues: always 0; later: the amount of taxed tier-0 ingredients you gain from learning this spell
            let castable = parse!(inputs[9], i32);
            let repeatable = parse!(inputs[10], i32); // for the first two leagues: always 0; later: 1 if this is a repeatable player spell

            match action_type {
                BREW => {
                    potions.push(Potion {
                        id: action_id,
                        price,
                        delta
                    });
                }
                CAST => {
                    spells.push(Spell {
                        id: action_id,
                        delta,
                        exhausted: castable == 0
                    })
                }
                _ => {}
            }
        }

        let mut players = [Player { ..Default::default() }, Player { ..Default::default() }];
        for i in 0..2 as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let inputs = input_line.split(" ").collect::<Vec<_>>();
            players[i] = Player {
                ingredients: Delta {val: [parse!(inputs[0], i32), parse!(inputs[1], i32), parse!(inputs[2], i32), parse!(inputs[3], i32)]},
                score: parse!(inputs[4], i32),
            };
        }
        let me = &players[0];
        let enemy = &players[1];

        let ranked_spells: Vec<(i32, Spell)> = spells.into_iter().map(|s| (s.base_value(me, enemy), s)).collect();
        let mut ranked_potions: Vec<(i32, Potion)> = potions.into_iter().map(|p| (p.base_value(me, enemy), p)).collect();

        ranked_potions.sort_by(|a, b| b.1.price.cmp(&a.1.price));

        let mut context_spell: Vec<(i32, Spell)> = ranked_spells.into_iter()
            .map(|pair|
                (pair.1.context_value(pair.0, me, enemy, &ranked_potions),
                 pair.1))
            .collect();

        context_spell.sort_by(|a, b| b.0.cmp(&b.0));

        let rest_score: i32 = context_spell.iter().filter(|pair| pair.1.exhausted).map(|pair| pair.0).sum();
        let rest_command = Command {
            action: Action::REST,
            id: 0,
            score: rest_score
        };

        let best_castable_spell: Option<(i32, Spell)> = context_spell.into_iter().filter(|pair| castable(me, &pair.1)).max_by(|a, b| b.0.cmp(&a.0));

        let cast_command = if best_castable_spell.is_some() {
            let spell = best_castable_spell.unwrap();
            eprintln!("castable: {:?}", &spell);
            Command {
                action: CAST,
                id: spell.1.id,
                score: spell.0,
            }
        } else {
            Command::default()
        };


        let best_brewable_popo: Option<(i32, Potion)> = ranked_potions.into_iter().filter(|pair| me.has_ingredients(&pair.1.delta)).max_by(|a, b| b.0.cmp(&a.0));
        let brew_command = if best_brewable_popo.is_some() {
            let popo = &best_brewable_popo.unwrap();
            Command {
                action: BREW,
                id: popo.1.id,
                score: popo.0,
            }
        } else {
            Command::default()
        };

        for x in vec![&brew_command, &cast_command, &rest_command] {
            eprintln!("CMD: {}", x);
        }
        let command: Command = vec![brew_command, cast_command, rest_command].into_iter().max_by(|a, b| a.score.cmp(&b.score)).unwrap();
        command.execute()
    }
}

fn castable(me: &Player, spell: &Spell) -> bool {
    !spell.exhausted && me.has_ingredients(&spell.delta) && me.has_space_for(&spell.delta)
}

trait Score {
    fn base_value(&self, me: &Player, enemy: &Player) -> i32;
    fn context_value(&self, my_base: i32, me: &Player, enemy: &Player, ranked_potions: &Vec<(i32, Potion)>) -> i32;
}

impl Score for Spell {
    fn base_value(&self, me: &Player, enemy: &Player) -> i32 {
        let mut basic_val = 1;
        for i in 0..INGREDIENTS_TIER {
            let room_left = 10 - me.ingredients.val[i];
            basic_val = basic_val + room_left.min(self.delta.val[i]);
            if room_left < 0 {
                basic_val = -10
            }
        }
        basic_val
    }

    fn context_value(&self, my_base: i32, me: &Player, enemy: &Player, ranked_potions: &Vec<(i32, Potion)>) -> i32 {
        let cant_do: Vec<&(i32, Potion)> = ranked_potions.into_iter().filter(|r| !me.has_ingredients(&r.1.delta)).collect();
        let mut adjusted_val = my_base;
        for pair in cant_do {
            for i in 0..INGREDIENTS_TIER {
                let needed: bool = (me.ingredients.val[i] + pair.1.delta.val[i]) < 0;
                if needed && self.delta.val[i] > 0 {
                    adjusted_val += pair.0;
                }
            }
        }
        adjusted_val
    }
}
impl Score for Potion {
    fn base_value(&self, me: &Player, enemy: &Player) -> i32 {
        self.price
    }

    fn context_value(&self, my_base: i32, me: &Player, enemy: &Player, ranked_potions: &Vec<(i32, Potion)>) -> i32 {
        if me.score > enemy.score {
            my_base * 2
        } else {
            my_base
        }
    }
}

impl Player {
    fn has_ingredients(&self, delta: &Delta) -> bool {
        for i in 0..INGREDIENTS_TIER {
            if self.ingredients.val[i] + delta.val[i] < 0 {
                return false
            }
        }
        true
    }
    fn has_space_for(&self, delta: &Delta) -> bool {
        let mut total: usize = 0;
        for i in 0..INGREDIENTS_TIER {
            total += (self.ingredients.val[i] + delta.val[i]) as usize;
        }
        total <= MAX_INGREDIENTS
    }
}
impl Command {
    // TODO: improve action enum
    fn execute(&self) {
        match self.action {
            BREW => println!("{} {}", BREW.sysout(), self.id),
            CAST => println!("{} {}", CAST.sysout(), self.id),
            _ => println!("{}", self.action.sysout())
        }
    }
}
impl Display for Command {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "ACTION: {}, SCORE: {}", self.action, self.score)
    }
}
impl Display for Spell {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "id: {} -- delta: {} -- exhausted: {}", self.id, self.delta, self.exhausted)
    }
}
impl Display for Delta {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "*[* {} {} {} {} *]*", &self.val[0], &self.val[1], &self.val[2], &self.val[3])
    }
}
impl Display for Action {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "!! {} !!", &self.sysout())
    }
}
impl Display for Player {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "PP inventory: {}", self.ingredients)
    }
}
impl Default for Command {
    fn default() -> Command {
        Command {
            action: Action::REST,
            id: 0,
            score: -999
        }
    }
}
impl Default for Player {
    fn default() -> Player {
        Player {
            ingredients: Delta{val: [0, 0, 0, 0]},
            score: 0
        }
    }
}
impl Potion {
    fn output(&self) -> String {
        self.id.to_string()
    }
}
impl Action {
    // TODO: there is probably a better way to do this
    fn sysout(&self) -> String {
        match &self {
            BREW => String::from("BREW"),
            CAST => String::from("CAST"),
            REST => String::from("REST"),
            WAIT => String::from("WAIT"),
        }
    }
    fn convert(str: &str) -> Action {
        match str {
            "BREW" => BREW,
            "REST" => REST,
            "CAST" => CAST,
            "WAIT" => WAIT,
            _ => WAIT,
        }
    }
}
