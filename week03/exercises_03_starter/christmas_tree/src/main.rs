use serde::Deserialize;
use std::collections::VecDeque;
use std::io;

#[derive(Debug, Deserialize)]
enum Instruction {
    Set(i32),
    Left,
    Right,
    Reset,
}

#[derive(Debug)]
struct Light {
    left: Option<Box<Light>>,
    right: Option<Box<Light>>,
    brightness: i32,
}

fn get_instructions_from_stdin() -> VecDeque<Instruction> {
    let mut instructions = String::new();
    io::stdin().read_line(&mut instructions).unwrap();
    ron::from_str(&instructions).unwrap()
}

fn main() {
    let instructions = get_instructions_from_stdin();
    let mut light = Light {
        left: None,
        right: None,
        brightness: 0,
    };
    let mut current_light: *mut Light = &mut light;
    let root: *mut Light = &mut light;

    for commands in instructions {
        match commands {
            Instruction::Set(brightness) => unsafe {
                (*current_light).brightness = brightness;
            },
            Instruction::Left => unsafe {
                if (*current_light).left.is_none() {
                    (*current_light).left = Some(Box::new(Light {
                        left: None,
                        right: None,
                        brightness: 0,
                    }));
                }
                current_light = (*current_light).left.as_mut().unwrap().as_mut();
            },
            Instruction::Right => unsafe {
                if (*current_light).right.is_none() {
                    (*current_light).right = Some(Box::new(Light {
                        left: None,
                        right: None,
                        brightness: 0,
                    }));
                }
                current_light = (*current_light).right.as_mut().unwrap().as_mut();
            },
            Instruction::Reset => {
                current_light = root;
            }
        }
    }

    let average = calculate_average_brightness(&light);
    println!("{}", average);
}

fn calculate_average_brightness(light: &Light) -> i32 {
    let (total_brightness, count) = get_brightness_sum_and_count(light);
    if count == 0 {
        0
    } else {
        total_brightness / count
    }
}

fn get_brightness_sum_and_count(light: &Light) -> (i32, i32) {
    let mut total = light.brightness;
    let mut count = 1;

    // Add left subtree
    if let Some(ref left_child) = light.left {
        let (left_sum, left_count) = get_brightness_sum_and_count(left_child);
        total += left_sum;
        count += left_count;
    }

    // Add right subtree
    if let Some(ref right_child) = light.right {
        let (right_sum, right_count) = get_brightness_sum_and_count(right_child);
        total += right_sum;
        count += right_count;
    }

    (total, count)
}
