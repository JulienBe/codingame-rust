use std::io;
use std::cmp;

/**
 * The goal for your program is to safely land the "Mars Lander" shuttle, the landing ship which contains the Opportunity rover.
 * Mars Lander is guided by a program, and right now the failure rate for landing on the NASA simulator is unacceptable.
 *
 * This puzzle is the second level of the "Mars Lander" trilogy. In this special puzzle, you need to minimize the quantity of fuel used.
 *
 * Built as a game, the simulator puts Mars Lander on a limited zone of Mars sky.
 * The zone is 7000m wide and 3000m high.
 * There is a unique area of flat ground on the surface of Mars, which is at least 1000 meters wide.
 *
 * Every second, depending on the current flight parameters (location, speed, fuel ...), the program must provide the new desired tilt angle and thrust power of Mars Lander:
 * Angle goes from -90° to 90° . Thrust power goes from 0 to 4 .
 *
 * The game simulates a free fall without atmosphere. Gravity on Mars is 3.711 m/s².
 * For a thrust power of X, a push force equivalent to X m/s² is generated and X liters of fuel are consumed.
 * As such, a thrust power of 4 in an almost vertical position is needed to compensate for the gravity on Mars.
 *
 * For a landing to be successful, the ship must:
 *     land on flat ground
 *     land in a vertical position (tilt angle = 0°)
 *     vertical speed must be limited ( ≤ 40m/s in absolute value)
 *     horizontal speed must be limited ( ≤ 20m/s in absolute value)
 *
 *
 * rotate is the desired rotation angle for Mars Lander. Please note that for each turn the actual value of the angle is limited to the value of the previous turn +/- 15°.
 * power is the desired thrust power. 0 = off. 4 = maximum power. Please note that for each turn the value of the actual power is limited to the value of the previous turn +/- 1.
 *
 * */

const GRAVITY: f32 = 3.711;

struct RoverState {
  x: i32,
  y: i32,
  hs: i32,
  // the horizontal speed (in m/s), can be negative.
  vs: i32,
  // the vertical speed (in m/s), can be negative.
  f: i32,
  // the quantity of remaining fuel in liters.
  r: i32,
  // the rotation angle in degrees (-90 to 90).
  p: i32, // the thrust power (0 to 4).
}

struct LandSection {
  left: Point,
  right: Point,
  flat: bool,
  gradient: f32,
}

#[derive(Clone, Copy)]
struct Point {
  x: i32,
  y: i32,
}

impl LandSection {
  pub fn new(left: Point, right: Point) -> Self {
    Self {
      left,
      right,
      flat: (&left.y == &right.y),
      gradient: (&right.x - &left.x) as f32 / (&right.y - &left.y) as f32,
    }
  }
  pub fn dst_from(&self, x: i32) -> i32 {
    cmp::min((x - self.left.x).abs(), (x - self.right.x).abs())
  }
}

enum Stage {
  Approach,
  Stabilize,
  SlowDown,
  Land,
}

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

/**
 * Save the Planet.
 * Use less Fossil Fuel.
 **/
fn main() {
  /***********
   * I N I T *
   ***********/
  // The program must first read the initialization data from standard input.
  // Then, within an infinite loop, the program must read the data from the standard input related to Mars Lander's current state and provide to the standard output the instructions to move Mars Lander.
  let mut input_line = String::new();
  io::stdin().read_line(&mut input_line).unwrap();
  let n = parse_input!(input_line, i32); // the number of points used to draw the surface of Mars.

  // A couple of integers landX landY providing the coordinates of a ground point.
  // By linking all the points together in a sequential fashion, you form the surface of Mars which is composed of several segments.
  // For the first point, landX = 0 and for the last point, landX = 6999
  let mut land: Vec<LandSection> = Vec::new();
  let mut last_point = get_surface_points();
  for i in 1..n as usize {
    let current_point = get_surface_points();
    let section = LandSection::new(last_point, current_point);
    land.push(section);
    last_point = current_point;
  }

  let mut state = get_state();
  let mut flat_sections: Vec<&LandSection> = land.iter().filter(|s| s.flat).collect();
  flat_sections.sort_by(|a, b| a.dst_from(state.x).cmp(&b.dst_from(state.x)));
  let target_section: &LandSection = flat_sections.first().unwrap();

  /***********
   * L O O P *
   ***********/
  let mut loops = 0; // used to stabilize
  let mut stage = Stage::Approach;
  let mut approach_from_left = false;

  loop {
    loops += 1;
    if state.vs > 0 {
      state.vs = state.vs * -1;
    }
    let aoa = (state.vs as f32).atan2(state.hs as f32);
    let vector_mag = (state.vs as f32 * 1.0).hypot(state.hs as f32);

    let mut dist = (
    ((vector_mag.powf(2.0)) / 2.0 * GRAVITY) *
        (1.0 + (
            1.0 + (
            (2.0 * GRAVITY * (state.y - target_section.left.y) as f32) / ((vector_mag.powf(2.0)) * (aoa.sin().powf(2.0))))
        ).powf(0.5))
    ) * (aoa * 2.0).sin();
    dist = ((dist) / 10.0) * -1.0;
    let mut desired_angle = 0;
    let mut desired_thrust = 0;
    match &stage {
      Stage::Approach => {
        desired_thrust = 4;
        if state.x < target_section.left.x {
          approach_from_left = true;
          desired_angle = -40;          
        } else if state.x > target_section.right.x {
          approach_from_left = false;
          desired_angle = 40;
        }
        if state.hs.abs() > 40 {
          stage = Stage::Stabilize;
        }
      },
      Stage::Stabilize => {
        if loops % 20 == 0 {
          desired_thrust = 3;
        } else {
          desired_thrust = 4;
        }
        let dist: i32 = ((dist * 0.3) * (state.hs.abs() as f32 * 0.01)) as i32;
        if approach_from_left && dist + state.x > target_section.left.x {
            stage = Stage::SlowDown;
        }
        if !approach_from_left && dist + state.x < target_section.right.x {
            stage = Stage::SlowDown;
        }
      },
      Stage::SlowDown => {
        desired_thrust = 4;
        let mut opp_aoa: i32 = (aoa.to_degrees() - 90.0) as i32;
        if opp_aoa < 0 {
          opp_aoa += 360;
        }
        opp_aoa %= 360;
        opp_aoa -= 180;
        opp_aoa = opp_aoa.clamp(-70, 70);

        if state.hs > 1 {
          desired_angle = opp_aoa;
        } else if state.hs < -1 {
          desired_angle = opp_aoa;
        } else {
          desired_angle = 0;
          stage = Stage::Land;  
        }
      }
      Stage::Land => {
        if state.vs < -30 {
          desired_thrust = 4;
        } else {
          desired_thrust = 3;
        }
      }
    }
    // Write an action using println!("message...");
    // To debug: eprintln!("Debug message...");
    // R P. R is the desired rotation angle. P is the desired thrust power.
    println!("{} {}", desired_angle, desired_thrust);
    state = get_state();
  }
}

fn get_state() -> RoverState {
  let mut input_line = String::new();
  io::stdin().read_line(&mut input_line).unwrap();
  let inputs = input_line.split(" ").collect::<Vec<_>>();
  let x = parse_input!(inputs[0], i32);
  let y = parse_input!(inputs[1], i32);
  let hs = parse_input!(inputs[2], i32); // the horizontal speed (in m/s), can be negative.
  let vs = parse_input!(inputs[3], i32); // the vertical speed (in m/s), can be negative.
  let f = parse_input!(inputs[4], i32); // the quantity of remaining fuel in liters.
  let r = parse_input!(inputs[5], i32); // the rotation angle in degrees (-90 to 90).
  let p = parse_input!(inputs[6], i32); // the thrust power (0 to 4).
  RoverState {
    x,
    y,
    hs,
    vs,
    f,
    r,
    p,
  }
}

fn get_surface_points() -> Point {
  let mut input_line = String::new();
  io::stdin().read_line(&mut input_line).unwrap();
  let inputs = input_line.split(" ").collect::<Vec<_>>();
  Point {
    x: parse_input!(inputs[0], i32), // X coordinate of a surface point. (0 to 6999)
    y: parse_input!(inputs[1], i32), // Y coordinate of a surface point. By linking all the points together in a sequential fashion, you form the surface of Mars.
  }
}
