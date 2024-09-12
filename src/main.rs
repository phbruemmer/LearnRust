use std::io;
use rand::Rng;

struct Robot {
    coordinates: [i16; 2]
}

struct Coins {
    coins: i8,
    coin_coordinates: [i16; 2],
}

struct Map {
    map: [u16; 2],
    obstacles: Vec<u16>
}

fn random(x: i16, y: i16) -> i16 { // Creates random number between x and y
    let mut rng = rand::thread_rng();
    rng.gen_range(x..y)
}


impl Coins {
    fn increment_coins(&mut self) { self.coins += 1 }

    fn change_coin_coordinates(&mut self, player_coordinates: &[i16; 2], map_size: &[u16; 2]) {
        let mut coordinates: [i16; 2] = [random(0, map_size[0] as i16), random(0, map_size[1] as i16)];

        while coordinates == *player_coordinates {
            coordinates = [random(0, map_size[0] as i16), random(0, map_size[1] as i16)];
        }
        self.coin_coordinates = coordinates;
    }
}


impl Robot {
    fn move_forward(&mut self, steps: i8, direction: &str) {
        if direction == "y" {
            self.coordinates[1] += steps as i16;
        } else if direction == "x" {
            self.coordinates[0] += steps as i16;
        } else {
            println!("[info] No valid direction!")
        }
    }
}


fn game(map: &mut Map, robot: &Robot, coins:  &mut Coins) {
    print!("{}[2J", 27 as char); // Clear terminal window

    if robot.coordinates == coins.coin_coordinates {
        coins.increment_coins();
        coins.change_coin_coordinates(&[random(1, map.map[0] as i16 - 1), random(1, map.map[1] as i16 - 1)],  &map.map);
        println!("Coin collected!");
    }

    println!("coins: {}\n\n", coins.coins);

    for row in 0..map.map[1] {
        for column in 0..map.map[0] {
            if column == robot.coordinates[0] as u16 && row == robot.coordinates[1] as u16 { // Places the robot character ("+") on the given coordinates
                print!(" + ");
            } else if column == coins.coin_coordinates[0] as u16 && row == coins.coin_coordinates[1] as u16 { // Places the coin character ("o") on the given coordinates
                print!(" o " );
            }  else {
                print!(" # ");
            }
        }
        print!("\n");
    }
}

fn start_game(_map: &mut Map, _robot: &mut Robot, _player: &mut Coins) {
    loop {
        loop {
            let mut input = String::new();
            let mut direction = String::new();
            println!("Please enter the direction (x / y): ");
            io::stdin()
                .read_line(&mut direction)
                .expect("Failed to read line.");
            let direction = direction.trim();

            println!("Please enter the amount of steps the robot has to go: ");
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line.");
            match input.trim().parse::<i8>() {
                Ok(num) => {
                    _robot.move_forward(num, &direction);
                    break;
                }
                Err(_) => {
                    println!("Invalid integer!");
                }
            }
        }
        println!("\n\n\n");
        game(_map, _robot, _player,);
    }
}

fn prepare_game() {
    let mut map_width = String::new();
    let mut map_height = String::new();

    println!("[INPUT] Enter Map width: ");
    io::stdin()
        .read_line(&mut map_width)
        .expect("Failed to read line.");

    println!("[INPUT] Enter Map height: ");
    io::stdin()
        .read_line(&mut map_height)
        .expect("Failed to read line.");

    match map_width.trim().parse::<u16>() {
        Ok(map_x) => {
            println!("Map width is valid.");
            match map_height.trim().parse::<u16>() {
                Ok(map_y) => {
                    println!("Map height is valid.");
                    let mut _map = Map {map: [map_x , map_y], obstacles: Vec::new() };
                    let mut _robot = Robot {coordinates: [0, 0]};
                    let mut _coins = Coins {coins: 0, coin_coordinates: [random(0, _map.map[0] as i16 - 1), random(0, _map.map[1] as i16 - 1)] ,};
                    start_game(&mut _map, &mut _robot, &mut _coins);
                }
                Err(_) => { main() }
            }

        }
        Err(_) => { prepare_game() }
    }
}

fn main() {
    prepare_game();
}




/*
fn main() {
    let result: String = Solution::add_binary("10100010011111".to_string(), "1100010100011".to_string());
    println!("{}", result)
}

struct Solution;

impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let mut a = &a;
        let mut b = &b;
        if a.len()<b.len() {
            std::mem::swap(&mut a, &mut b);
        }

        let res = a
            .as_bytes().rchunks(127).zip(
            b.as_bytes().rchunks(127).chain(std::iter::repeat(&[b'0'; 127][..])),
        )
            .fold((String::new(), 0), |s, (a, b)| {
                let sum = unsafe {
                    s.1 + u128::from_str_radix(std::str::from_utf8_unchecked(a), 2).unwrap_or(0)
                        + u128::from_str_radix(std::str::from_utf8_unchecked(b), 2).unwrap_or(0)
                };
                (format!("{:0127b}", sum & 0x7fffffffffffffffffffffffffffffff) + &s.0, sum >> 127)
            });

        if res.1 == 1 {
            "1".to_string() + &res.0
        } else {
            let str = res.0.trim_start_matches("0").to_string();
            if str.len() > 0 {
                str
            } else {
                "0".to_string()
            }
        }
    }
}

*/

/*

use std::io::{stdin,stdout,Write};


fn main() {
    let logical: bool = false;
    println!("First Output!");

    println!("Base 10:                  {}", 69420);    // 69420
    println!("Base 2 (binary)           {:b}", 69420);  // 1000111100101100
    println!("Base 8 (octal)            {:o}", 69420);  // 207454
    println!("Base 16 (hexadecimal)     {:x}", 69420);  // 10f2c

    println!("{number:_<5}", number=1);
    println!("{number:_>5}", number=1);

    if logical {
        println!("Variable logical is True!")
    } else {
        println!("Variable logical is False!")
    }
    get_user_input();
}


fn get_user_input() {
    loop {
        let mut s = String::new();
        println!("Do you want to start the calculator? [Y/N]");
        stdout().flush().unwrap();  // Ensures prompt is displayed immediately
        stdin().read_line(&mut s).expect("Did not enter a correct string");

        // Trim the input to remove any newline characters or extra spaces
        let s = s.trim();

        if s == "Y" || s == "y" {
            calculator();
        } else if s == "N" || s == "n" {
            println!("Exiting...");
            break;
        } else {
            println!("Invalid input. Please enter Y or N.");
        }
    }
}


fn calculator() {
    let mut input = String::new(); // Temporary string to hold input
    let num1: i128;
    let num2: i128;

    // Reading the first number
    print!("Enter first number: ");
    stdout().flush().unwrap();  // Ensures the prompt is displayed immediately
    stdin().read_line(&mut input).expect("Failed to read line");
    num1 = input.trim().parse().expect("Invalid number");

    // Clear the input buffer
    input.clear();

    // Reading the second number
    print!("Enter second number: ");
    stdout().flush().unwrap();
    stdin().read_line(&mut input).expect("Failed to read line");
    num2 = input.trim().parse().expect("Invalid number");

    // Calculating the solution
    let solution = num1 + num2;

    // Printing the solution
    println!("The solution is: {}", solution);
}



*/