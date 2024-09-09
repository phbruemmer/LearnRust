use std::io;

struct Robot {
    x: i8,
    y: i8,
}

impl Robot {
    pub fn move_forward(&mut self, steps: i8, direction: &str) {
        if direction == "y" {
            self.y += steps;
        } else if direction == "x" {
            self.x += steps;
        } else {
            println!("[info] No valid direction!")
        }
    }

    pub fn get_x(&mut self) -> i8 { self.x }
    pub fn get_y(&mut self) -> i8 { self.y }

    pub fn get_current_position(&mut self) {
        println!("I am on x coordinate: {}", self.x);
        println!("And on y coordinate: {}", self.y);
    }
}


fn game(robot_x: i8, robot_y: i8) {
    let map = [["#", "#", "#", "#", "#", "#", "#", "#"],
        ["#", "#", "#", "#", "#", "#", "#", "#"],
        ["#", "#", "#", "#", "#", "#", "#", "#"],
        ["#", "#", "#", "#", "#", "#", "#", "#"],
        ["#", "#", "#", "#", "#", "#", "#", "#"],
        ["#", "#", "#", "#", "#", "#", "#", "#"],
        ["#", "#", "#", "#", "#", "#", "#", "#"],
        ["#", "#", "#", "#", "#", "#", "#", "#"]];
    let mut current_x: i8 = 0;
    let mut current_y: i8 = 0;

    for row in map.iter() {
        for column in row.iter() {
            if current_x == robot_x && current_y == robot_y {
                print!(" + ")
            } else {
                print!(" {} ", column);
            }
            current_x += 1
        }
        print!("\n");
        current_x = 0;
        current_y += 1;
    }
}

fn main() {
    let mut robot = Robot {x: 0, y: 0};
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
                    robot.move_forward(num, &direction);
                    break;
                }
                Err(_) => {
                    println!("Invalid integer!");
                }
            }
        }
        println!("\n\n\n");
        game(robot.get_x(), robot.get_y());
    }
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