use std::io::stdin;
use std::collections::{HashMap, HashSet};

pub mod effect_to_path;

fn main() {

    println!("Welcome to the CLI Version of the Schedule 1 Mixing Calculator.");
    loop {
        println!("Select a command\n\
        1. Find paths with desired effects and ingredients");

        let mut command = String::new();
        stdin().read_line(&mut command).expect("stdin failed");
        match command.trim().parse() {
            Err(_) => {
                println!("Invalid integer!");
                continue;
            },
            Ok(1) => {
                handle_effect_to_path();
            },
            Ok(n) => {
                println!("Not a command!");
                continue;
            }
        }
    }

}

fn handle_effect_to_path() {

    let effect_map: HashMap<u8, &str> = HashMap::from([
        (1, "Energizing"),
        (2, "Sedating"),
        (3, "Toxic"),
        (4, "Calorie-Dense"),
        (5, "Athletic"),
        (6, "Balding"),
        (7, "Slippery"),
        (8, "Gingeritis"),
        (9, "Spicy"),
        (10, "Jennerising"),
        (11, "Sneaky"),
        (12, "Tropic Thunder"),
        (13, "Long Faced"),
        (14, "Foggy"),
        (15, "Thought-Provoking"),
        (16, "Bright Eyed"),
        (17, "Euphoric"),
        (18, "Munchies"),
        (19, "Paranoia"),
        (20, "NONE, DO NOT PICK"), //Duplicate, changed some rules and broke some things.
        (21, "Cyclopean"),
        (22, "Laxative"),
        (23, "Calming"),
        (24, "Focused"),
        (25, "Electrifying"),
        (26, "Refreshing"),
        (27, "Shrinking"),
        (28, "Glowing"),
        (29, "Disorienting"),
        (30, "Smelly"),
        (31, "Anti-Gravity"),
        (32, "Seizure Inducing"),
        (33, "Schizophrenia"),
        (34, "Zombifying"),
        (35, "Explosive"),
    ]);

    let mixer_map: HashMap<u8, &str> = HashMap::from([
        (1, "Cuke"),
        (2, "Flu Medicine"),
        (3, "Gasoline"),
        (4, "Donut"),
        (5, "Energy Drink"),
        (6, "Mouth Wash"),
        (7, "Motor Oil"),
        (8, "Banana"),
        (9, "Chili"),
        (10, "Iodine"),
        (11, "Paracetemol"),
        (12, "Viagra"),
        (13, "Horse Semen"),
        (14, "Mega Bean"),
        (15, "Addy"),
        (16, "Battery"),
    ]);


    println!("Effects:");
    let mut i: u8 = 1;
    while let Some(v) = effect_map.get(&i) {
        println!("{} - {}", i, v);
        i += 1;
    }
    println!("PS: I am aware there is a number missing! Ignore it!\n");

    let mut effect_string = String::new();
    println!("Please enter a list of the desired effect IDs, seperated by spaces: ");
    stdin().read_line(&mut effect_string).unwrap();

    let mut desired_effects: Vec<u8> = Vec::new();
    effect_string.split_whitespace().for_each(|e| {
        desired_effects.push(e.parse().unwrap());
    });

    desired_effects.sort();

    println!("1. Use all possible ingredients\n2. Select a specific set");

    let mut desired_ingredients: HashSet<u8> = HashSet::new();
    let mut ingredient_string = String::new();
    stdin().read_line(&mut ingredient_string).unwrap();
    match ingredient_string.trim().parse() {
        Ok(1) => {
            for i in 1..=16 {
                desired_ingredients.insert(i);
            }
        },
        Ok(2) => {
            println!("Ingredients:");
            let mut i: u8 = 1;
            while let Some(v) = mixer_map.get(&i) {
                println!("{} - {}", i, v);
                i += 1;
            }
            let mut mixer_string = String::new();
            println!("Please enter a list of the desired ingredients, seperated by spaces: ");
            stdin().read_line(&mut mixer_string).unwrap();
            mixer_string.split_whitespace().for_each(|m| {
                desired_ingredients.insert(m.parse().unwrap());
            });
        }
        Ok(n) => {
            println!("Invalid.");
            return;
        },
        Err(_) => {
            println!("Invalid.");
            return;
        }
    }

    println!("Enter your starting effect (for example Og Kush starts as calming). If your drug does not start with an effect, ie meth, just hit enter: ");
    let mut starting_effect_string = String::new();
    let starting_effect: Option<u8>;
    stdin().read_line(&mut starting_effect_string).unwrap();
    if starting_effect_string.trim().len() == 0 {
        starting_effect = None;
    } else {
        starting_effect = Some(starting_effect_string.trim().parse().unwrap());
    }

    println!("How many paths should be shown: ");
    let mut path_string = String::new();
    stdin().read_line(&mut path_string).unwrap();
    let path_count: i32 = path_string.trim().parse().unwrap();

    println!("Search being ran, this may take a minute...");
    effect_to_path::run(desired_effects, desired_ingredients, starting_effect, path_count);
}