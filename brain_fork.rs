use std::io;
use std::time::Instant;

const POSSIBILITIES_TO_CONSIDER: usize = 4;
const A: isize = 97;
const NB_ZONES: usize = 30;
const NB_RUNES: usize = 27;
const RUNE_UP: Roll =   Roll { c: '+', };
const RUNE_DOWN: Roll = Roll { c: '-', };
const ZONE_UP: Roll =   Roll { c: '>', };
const ZONE_DOWN: Roll = Roll { c: '<', };

#[derive(Debug)]
struct Roll {
  c: char,
}
#[derive(Copy, Clone, Debug)]
struct Ring {
  top: isize,
  current: isize,  
}
impl Ring {
  fn set_to<'a>(&mut self, target: isize, roll_down: &'a Roll, roll_up: &'a Roll) -> Move<'a> {
    let mut dist_down = (self.current - target, roll_down);
    let mut dist_up = (target - self.current, roll_up);    
    if dist_down.0 < 0 {
      dist_down.0 += self.top;
    }
    if dist_up.0 < 0 {
        dist_up.0 += self.top;
    }    
    if dist_up.0 < dist_down.0 {
      Move {
        times: dist_up.0 as usize,
        roll: roll_up,  
      }  
      //dist_up.1.c.repeat(dist_up.0 as usize)
    } else {
      Move {
        times: dist_down.0 as usize,
        roll: roll_down,  
      }  
      //dist_down.1.c.repeat(dist_down.0 as usize)
    }
  }
}
#[derive(Copy, Clone, Debug)]
struct Move<'m> {
  times: usize,
  roll: &'m Roll,
}
#[derive(Copy, Clone, Debug)]
struct Terrain {
  runes: [Ring; NB_ZONES],
  zones: Ring,
}
impl Terrain {
  fn new() -> Terrain {
    Terrain {
      runes: [Ring { top: (NB_RUNES as isize), current: 0 }; NB_ZONES],
      zones: Ring { top: NB_ZONES as isize, current: 0},
    }  
  }
}
#[derive(Copy, Clone, Debug)]
struct Transition<'m> {
  rune_index: usize,
  letter_i: usize,
  zone_move: Move<'m>,  
  rune_move: Move<'m>,
}
#[derive(Debug)]
struct LookAhead<'m> {
  seed: &'m Transition<'m>,
  terrains_costs: Vec<(Terrain, usize)>,
}
impl LookAhead<'_> {
  fn new<'m>(seed: &'m Transition, base: Terrain) -> LookAhead<'m> {
    LookAhead {
      seed,
      terrains_costs: vec![(base, seed.score())],
    }  
  }
}
impl Transition<'_> {
  fn score(&self) -> usize {
    self.zone_move.times + self.rune_move.times  
  }
  fn to_string(&self) -> String {
    let mut z = self.zone_move.roll.c.to_string().repeat(self.zone_move.times);             // that .to_string() heres bothers me. Maybe move rolls outside of const
    z.push_str(self.rune_move.roll.c.to_string().repeat(self.rune_move.times).as_str());
    z
  }
}

fn main() {
  let mut input_line = String::new();
  io::stdin().read_line(&mut input_line).unwrap();
  let magic_phrase = input_line.trim_matches('\n').to_string();
  let mut index = 0;
  let mut target_string: String = String::new();
  eprintln!("input : {}", magic_phrase);

  let mut terrain = Terrain::new();  
   
  // ITERATE OVER EACH CHARACTER
  let start = Instant::now();
  while index < magic_phrase.chars().count() {
    let mvt = get_move(index, &magic_phrase, terrain);
    terrain = mvt.1;
    target_string.push_str(mvt.0.as_str());
    target_string.push_str(".");
    index += 1;
  }

  eprintln!("Time: {:?}", start.elapsed());
  println!("{}", target_string);
}

fn get_move(phrase_index: usize, phrase: &String, terrain: Terrain) -> (String, Terrain) {
  let letter_i = get_char_index_at(phrase_index, &phrase);
  let seeds = best_transitions(&terrain, letter_i);  
  
  let mut look_ahead: Vec<LookAhead> = seeds.iter()
    .map(|seed| LookAhead::new(seed, terrain))
    .collect();
  
  /* That needs work 
  // depth
  if phrase_index % 5 == 0 {
    for n in 1..100 {
      if phrase_index + n < phrase.char_indices().count() {
        let letter_i_plus = get_char_index_at(phrase_index + n, &phrase);
        // each path
        look_ahead.iter_mut().for_each(|mut seed| {
          let mut next_tc = Vec::new();        
          // branch to next paths
          seed.terrains_costs.iter().for_each(|terr_cost| {
            let transitions = best_transitions(&terr_cost.0, letter_i_plus);
            transitions.iter().for_each(|trans| {
              let new_cost = trans.score() + terr_cost.1;
              next_tc.push((get_next_state(trans, &terr_cost.0), new_cost));
            });      
          });
          next_tc.sort_unstable_by(|a, b| a.1.cmp(&b.1));
          seed.terrains_costs = if next_tc.len() > 50 {
            next_tc.split_at(50).0.to_vec()  
          } else {
            next_tc  
          };        
        });
      }  
    }
  }
  look_ahead.iter_mut().for_each(|l_a| {
    l_a.terrains_costs.sort_by(|a, b| a.1.cmp(&b.1))
  });
  look_ahead.sort_unstable_by(|a, b| a.terrains_costs.get(0).unwrap().1.cmp(&b.terrains_costs.get(0).unwrap().1));
  */

  let selected: &LookAhead = look_ahead.get(0).unwrap();
  (selected.seed.to_string(), get_next_state(&selected.seed, &terrain))
}

fn get_char_index_at(i: usize, phrase: &String) -> isize {
  let current_char: isize = phrase.chars().nth(i).unwrap().to_ascii_lowercase() as isize;
  (current_char - (A - 1)).clamp(0, 255) // 0 to clamp spaces. 255 to avoid masking an error       
}

fn best_transitions(terrain: &Terrain, letter_i: isize) -> [Transition; POSSIBILITIES_TO_CONSIDER]{
  let mut transitions: Vec<Transition> = terrain.runes.iter().enumerate().map(|(i, z)| {
    Transition {
      letter_i: letter_i as usize,
      rune_index: i,
      zone_move: terrain.zones.clone().set_to(i as isize, &ZONE_DOWN, &ZONE_UP),
      rune_move: z.clone().set_to(letter_i, &RUNE_DOWN, &RUNE_UP),
    }
  }).collect();
  transitions.sort_unstable_by(|a, b| a.score().cmp(&b.score()));
  //transitions.split_at(POSSIBILITIES_TO_CONSIDER).0
  //*transitions[..(POSSIBILITIES_TO_CONSIDER - 1)]
  [*transitions.get(0).unwrap(), *transitions.get(1).unwrap(), *transitions.get(2).unwrap(), *transitions.get(3).unwrap()]
}

fn get_next_state(transition: &Transition, origin: &Terrain) -> Terrain {
  let mut next_state: Terrain = Terrain {
      runes: origin.runes,
      zones: origin.zones,
  };
  next_state.zones.current = transition.rune_index as isize;
  next_state.runes[next_state.zones.current as usize].current = transition.letter_i as isize;
  next_state
}
