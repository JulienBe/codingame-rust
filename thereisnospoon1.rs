use std::io;

macro_rules! print_err {
    ($($arg:tt)*) => (
        {
            use std::io::Write;
            writeln!(&mut ::std::io::stderr(), $($arg)*).ok();
        }
    )
}

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let width = parse_input!(input_line, i32); // the number of cells on the X axis
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let height = parse_input!(input_line, i32); // the number of cells on the Y axis
    
    let mut lines = Vec::new();
    for i in 0..height as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let line = input_line.trim_right().to_string(); // width characters, each either 0 or .
        lines.push(line);
    }

    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                // there is a cell
                '0' => {
                    let right = search(x, y, 1, 0, &lines);
                    let down = search(x, y, 0, 1, &lines);
                    println!("{} {} {} {}", x, y, right, down);
                },
                _ => {}
            }
        }
    }
}

fn search(x: usize, y: usize, offset_x: usize, offset_y: usize, lines: &Vec<String>) -> String {
    return if y >= lines.len() || x >= lines[y].len() {
        String::from("-1 -1")
    } else if has_cell(x + offset_x, y + offset_y, lines) {
        format!("{} {}", x + offset_x, y + offset_y)
    } else {
        search(x + offset_x, y + offset_y, offset_x, offset_y, lines)
    }
}

fn has_cell(x: usize, y: usize, lines: &Vec<String>) -> bool {
    return x >= 0 &&
            y >= 0 &&
            y < lines.len() &&
            x < lines[y].len()  &&
            lines[y].chars().nth(x).unwrap() == '0'
}
