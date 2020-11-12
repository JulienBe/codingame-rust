/*

Earn more rupees than your opponent!

The game takes place in a **potion shop**, in which two twin-sister witches are trying to prove they are the better potion brewer.

They have set up a contest: make more rupees selling potions than your sister.

However, the witch's hut in witch they set up shop is quite small, so they must share the same workspace, and deal with the same **client orders**.

# Rules

Each player controls a witch, each witch has access to their own inventory of potion ingredients.

Each client order is a list of ingredients required to brew a potion and earn some rupees.

The game is played over several rounds. Each player performs one action each turn, simultaneously.


#  Ingredients

There are 4 tiers of ingredient, indexed from 0 to 3.

Each witch starts with a full inventory of 10 ingredients.

The inventory is represented by inv: 4 numbers each representing the amount of each ingredient tier.

inv = [3,1,0,0]

# Action overview

For this league, you must Brew two potions from the list of client orders. The witch having earned the most rupees wins.

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

The game ends once at least one witch has brewed 2 potions.

The game stops automatically after 100 rounds.


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

use std::io;
use crate::Action::BREW;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

const INGREDIENTS_TIER: usize = 4;

enum Action {
    BREW,
}

struct Recipe {
    ingredients: [i32; 4],
}
struct Stock {
    ingredients: [i32; 4],
}

impl Stock {
    fn can_make(&self, recipe: &Recipe) -> bool {
        for i in 0..INGREDIENTS_TIER {
            if self.ingredients[i] < recipe.ingredients[i] {
                return false
            }
        }
        true
    }
}

struct Potion {
    id: i32,
    price: i32,
    recipe: Recipe,
}

impl Potion {
    fn output(&self) -> String {
        self.id.to_string()
    }
}

fn main() {

    // game loop
    loop {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let action_count = parse_input!(input_line, i32); // the number of spells and recipes in play
        let mut potions: Vec<Potion> = vec![];
        for i in 0..action_count as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let inputs = input_line.split(" ").collect::<Vec<_>>();
            let action_id = parse_input!(inputs[0], i32); // the unique ID of this spell or recipe
            let action_type = inputs[1].trim().to_string(); // in the first league: BREW; later: CAST, OPPONENT_CAST, LEARN, BREW
            let delta_0 = parse_input!(inputs[2], i32); // tier-0 ingredient change
            let delta_1 = parse_input!(inputs[3], i32); // tier-1 ingredient change
            let delta_2 = parse_input!(inputs[4], i32); // tier-2 ingredient change
            let delta_3 = parse_input!(inputs[5], i32); // tier-3 ingredient change
            let price = parse_input!(inputs[6], i32); // the price in rupees if this is a potion
            let tome_index = parse_input!(inputs[7], i32); // in the first two leagues: always 0; later: the index in the tome if this is a tome spell, equal to the read-ahead tax
            let tax_count = parse_input!(inputs[8], i32); // in the first two leagues: always 0; later: the amount of taxed tier-0 ingredients you gain from learning this spell
            let castable = parse_input!(inputs[9], i32); // in the first league: always 0; later: 1 if this is a castable player spell
            let repeatable = parse_input!(inputs[10], i32); // for the first two leagues: always 0; later: 1 if this is a repeatable player spell

            let receipe = Recipe { ingredients: [delta_0, delta_1, delta_2, delta_3] };
            let potion = Potion {
                id: action_id,
                price,
                recipe: receipe
            };
            potions.push(potion);
        }
        let mut my_stock = Stock {
            ingredients: [0, 0, 0, 0]
        };
        for i in 0..2 as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let inputs = input_line.split(" ").collect::<Vec<_>>();
            let inv_0 = parse_input!(inputs[0], i32); // tier-0 ingredients in inventory
            let inv_1 = parse_input!(inputs[1], i32);
            let inv_2 = parse_input!(inputs[2], i32);
            let inv_3 = parse_input!(inputs[3], i32);
            let score = parse_input!(inputs[4], i32); // amount of rupees
            if i == 0 {
                my_stock = Stock {
                    ingredients: [inv_0, inv_1, inv_2, inv_3]
                }
            }
        }

        // Write an action using println!("message...");
        // To debug: eprintln!("Debug message...");


        // in the first league: BREW <id> | WAIT; later: BREW <id> | CAST <id> [<times>] | LEARN <id> | REST | WAIT
        let action = BREW;
        let can_brew: Vec<Potion> = potions.into_iter().filter(|p| my_stock.can_make(&p.recipe)).collect();
        println!("BREW {}", can_brew.get(0).unwrap().output());
    }

}
