use std::io;

const A: isize = 97;
const NB_ZONES: isize = 30;
const NB_RUNES: isize = 27;

struct Roll<'a> {
  inc: isize,
  c: &'a str,
}
#[derive(Copy, Clone)]
struct Ring {
  top: isize,
  current: isize,
}
impl Ring {
  // in practice, only roll by +-1
  fn roll(&mut self, roll: &Roll) {
    if self.current == 0 && roll.inc < 0 {
      self.current = self.top;
    } else if self.current == self.top && roll.inc > 0 {
      self.current = 0;
    } else {
      self.current += roll.inc;
    }
  }

  fn set_to(&mut self, target: isize, roll_down: &Roll, roll_up: &Roll) -> String {
    let roll: &Roll = if target - self.current <= self.top / 2 {
      roll_up
    } else {
      roll_down
    };
    let mut accum: String = String::new();
    while target != self.current {
      self.roll(roll);
      accum.push_str(roll.c);
    }
    accum
  }
}

fn main() {
  let mut input_line = String::new();
  io::stdin().read_line(&mut input_line).unwrap();
  let magic_phrase = input_line.trim_matches('\n').to_string();
  let mut index = 0;
  let mut target_string: String = String::new();
  eprintln!("input : {}", magic_phrase);
  let rune_up =   Roll { inc: 1,  c: "+",  };
  let rune_down = Roll { inc: -1, c: "-",  };
  let zone_up =   Roll { inc: 1,  c: ">",  };
  let zone_down = Roll { inc: -1, c: "<",  };

  let mut zones: [Ring; NB_ZONES as usize] = [Ring { top: (NB_RUNES - 1), current: 0 }; NB_ZONES as usize];
  let mut zone_ring = Ring { top: NB_ZONES - 1, current: 0 };

  // ITERATE OVER EACH CHARACTER
  while index < magic_phrase.chars().count() {
    let current_char: isize = magic_phrase.chars().nth(index).unwrap().to_ascii_lowercase() as isize;
    let desired_rune_index = (current_char - (A - 1)).clamp(0, 255); // 0 to clamp spaces. 255 to avoid masking an error     

    // brute force all possibilities. Compute would be better
    let strs: Vec<(usize, String)> = zones.iter().enumerate().map(|(i, z)| {
      let mut accum: String = String::new();
      let zones_str = zone_ring.clone().set_to(i as isize, &zone_down, &zone_up);
      let rune_str = z.clone().set_to(desired_rune_index, &rune_down, &rune_up);
      accum.push_str(zones_str.as_str());
      accum.push_str(rune_str.as_str());
      (i, accum)
    }).collect();
    // pick min
    let min: &(usize, String) = strs.iter()
      .min_by(|x, y| x.1.chars().count().cmp(&y.1.chars().count()))
      .unwrap();
    zone_ring.current = min.0 as isize;
    zones[zone_ring.current as usize].current = desired_rune_index;
    // update final str
    target_string.push_str(min.1.as_str());
    target_string.push_str(".");

    index += 1;
  }

  // Write an action using println!("message...");
  // To debug: eprintln!("Debug message...");    
  println!("{}", target_string);
}
