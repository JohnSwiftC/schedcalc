use std::collections::{HashMap, HashSet, VecDeque};

struct MixNode {
    effs: Vec<u8>,
    path: Vec<u8>,
}

fn main() {
    // Touch these settings.
    let mut desired_effects: Vec<u8> = vec![1, 2, 12, 19];
    let desired_ingredients_vec: Vec<u8> =
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
    let mut desired_path_count = 5;

    let first_node = MixNode {
        effs: vec![23], // This is the initial effect for your ingredient. For example, for Og Kush, put 23 for the Calming initial effect.
        path: Vec::new(),
    };

    // Nothing below this for usage.
    desired_effects.sort();

    let mut desired_ingredients: HashSet<u8> = HashSet::new();

    for ing in desired_ingredients_vec {
        desired_ingredients.insert(ing);
    }

    let mut previous_nodes = HashSet::new(); // HashSet is sorted effects of a node.
    let mut stack = VecDeque::new();

    // Encodings

    // Big oopsies here, foggy is included twice. I am going to keep this like that for now.
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
        //(20, "Foggy"), Duplicate, changed some rules and broke some things.
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

    stack.push_back(first_node);

    while let Some(node) = stack.pop_front() {
        if previous_nodes.contains(&node.effs) {
            continue;
        } else {
            previous_nodes.insert(node.effs.clone());
        }
        if node.effs.len() > 8 {
            continue;
        }
        let mut contains_munchies = false;
        let mut munchies_mixed = false;

        if node.effs.contains(&18) {
            contains_munchies = true;
        }

        if desired_ingredients.contains(&1) {
            // Cuke
            let mut cuke_mix = Vec::new();
            for eff in &node.effs {
                match eff {
                    3 => inr(&mut cuke_mix, 17),  // Toxic → Euphoric
                    7 => inr(&mut cuke_mix, 18),  // Slippery → Munchies
                    11 => inr(&mut cuke_mix, 19), // Sneaky → Paranoia
                    14 => inr(&mut cuke_mix, 21), // Foggy → Cyclopean
                    8 => inr(&mut cuke_mix, 15),  // Gingeritis → Thought-Provoking
                    18 => inr(&mut cuke_mix, 5),  // Munchies → Athletic
                    17 => inr(&mut cuke_mix, 22), // Euphoric → Laxative
                    n if !(*n == 18 && munchies_mixed) => inr(&mut cuke_mix, *n), // Keep other effects unchanged
                    _ => (),
                };
            }

            inr(&mut cuke_mix, 1); // Add the main effect (Cuke)

            // Sort effects
            cuke_mix.sort();

            let new_node = MixNode {
                effs: cuke_mix,
                path: {
                    let mut path = node.path.clone();
                    path.push(1);
                    path
                },
            };

            if desired_effects
                .iter()
                .all(|&eff| new_node.effs.contains(&eff))
            {
                display_path(
                    &new_node.path,
                    &new_node.effs,
                    &mixer_map,
                    &effect_map,
                    &mut desired_path_count,
                );
            }

            stack.push_back(new_node);
        }

        if desired_ingredients.contains(&2) {
            // Flu
            let mut flu_mix = Vec::new();
            for eff in &node.effs {
                match eff {
                    23 => inr(&mut flu_mix, 16),
                    5 => inr(&mut flu_mix, 18),
                    15 => inr(&mut flu_mix, 8),
                    21 => inr(&mut flu_mix, 14),
                    18 => inr(&mut flu_mix, 7),
                    22 => inr(&mut flu_mix, 17),
                    17 => inr(&mut flu_mix, 3),
                    24 => inr(&mut flu_mix, 23),
                    25 => inr(&mut flu_mix, 26),
                    27 => inr(&mut flu_mix, 19),
                    n => inr(&mut flu_mix, *n),
                };
            }

            inr(&mut flu_mix, 2); // Add the main effect (Flu)

            // Sort effects
            flu_mix.sort();

            let new_node = MixNode {
                effs: flu_mix,
                path: {
                    let mut path = node.path.clone();
                    path.push(2);
                    path
                },
            };

            if desired_effects
                .iter()
                .all(|&eff| new_node.effs.contains(&eff))
            {
                display_path(
                    &new_node.path,
                    &new_node.effs,
                    &mixer_map,
                    &effect_map,
                    &mut desired_path_count,
                );
            }

            stack.push_back(new_node);
        }

        if desired_ingredients.contains(&3) {
            // Gasoline // RULE CHANGE! If energizing and euphoric are present, it will turn energizing into spicy and leave euphoric
            let mut gas_mix = Vec::new();
            for eff in &node.effs {
                match eff {
                    9 => inr(&mut gas_mix, 30),
                    10 => inr(&mut gas_mix, 11),
                    11 => inr(&mut gas_mix, 12),
                    18 => inr(&mut gas_mix, 2),
                    1 => inr(&mut gas_mix, 9),
                    17 => inr(&mut gas_mix, 1),
                    22 => inr(&mut gas_mix, 14),
                    29 => inr(&mut gas_mix, 28),
                    19 => inr(&mut gas_mix, 23),
                    25 => inr(&mut gas_mix, 29),
                    27 => inr(&mut gas_mix, 24),
                    n => inr(&mut gas_mix, *n), // Keep other effects unchanged
                };
            }

            inr(&mut gas_mix, 3); // Add the main effect (Gasoline)

            // Sort effects
            gas_mix.sort();

            let new_node = MixNode {
                effs: gas_mix,
                path: {
                    let mut path = node.path.clone();
                    path.push(3);
                    path
                },
            };

            if desired_effects
                .iter()
                .all(|&eff| new_node.effs.contains(&eff))
            {
                display_path(
                    &new_node.path,
                    &new_node.effs,
                    &mixer_map,
                    &effect_map,
                    &mut desired_path_count,
                );
            }

            stack.push_back(new_node);
        }

        if desired_ingredients.contains(&4) {
            // Donut
            let mut donut_mix = Vec::new();

            for eff in &node.effs {
                match eff {
                    4 => inr(&mut donut_mix, 35),  // Calorie-Dense -> Explosive
                    6 => inr(&mut donut_mix, 11), // Balding -> Sneaky // NUMBER WAS WRONG GO FUCK YOURSELF
                    31 => inr(&mut donut_mix, 7), // Anti-Gravity -> Slippery
                    10 => inr(&mut donut_mix, 8), // Jennerising -> Gingeritis
                    24 => inr(&mut donut_mix, 17), // Focused -> Euphoric
                    27 => inr(&mut donut_mix, 1), // Shrinking -> Energizing
                    n => inr(&mut donut_mix, *n), // Keep other effects unchanged
                };
            }

            inr(&mut donut_mix, 4); // Add the main effect (Donut)

            // Sort effects
            donut_mix.sort();
            let new_node = MixNode {
                effs: donut_mix,
                path: {
                    let mut path = node.path.clone();
                    path.push(4);
                    path
                },
            };

            if desired_effects
                .iter()
                .all(|&eff| new_node.effs.contains(&eff))
            {
                display_path(
                    &new_node.path,
                    &new_node.effs,
                    &mixer_map,
                    &effect_map,
                    &mut desired_path_count,
                );
            }

            stack.push_back(new_node);
        }

        if desired_ingredients.contains(&5) {
            // Energy Drink
            let mut energy_drink_mix = Vec::new();

            for eff in &node.effs {
                match eff {
                    2 => inr(&mut energy_drink_mix, 18),  // Sedating -> Munchies
                    17 => inr(&mut energy_drink_mix, 1),  // Euphoric -> Energizing
                    9 => inr(&mut energy_drink_mix, 17),  // Spicy -> Euphoric
                    12 => inr(&mut energy_drink_mix, 11), // Tropic Thunder -> Sneaky
                    28 => inr(&mut energy_drink_mix, 29), // Glowing -> Disorienting
                    14 => inr(&mut energy_drink_mix, 22), // Foggy -> Laxative
                    29 => inr(&mut energy_drink_mix, 25), // Disorienting -> Electrifying
                    33 => inr(&mut energy_drink_mix, 6),  // Schizophrenia -> Balding
                    24 => inr(&mut energy_drink_mix, 27), // Focused -> Shrinking
                    n => inr(&mut energy_drink_mix, *n),  // Keep other effects unchanged
                };
            }

            inr(&mut energy_drink_mix, 5); // Add the main effect (Energy Drink)

            // Sort effects
            energy_drink_mix.sort();
            let new_node = MixNode {
                effs: energy_drink_mix,
                path: {
                    let mut path = node.path.clone();
                    path.push(5);
                    path
                },
            };

            if desired_effects
                .iter()
                .all(|&eff| new_node.effs.contains(&eff))
            {
                display_path(
                    &new_node.path,
                    &new_node.effs,
                    &mixer_map,
                    &effect_map,
                    &mut desired_path_count,
                );
            }

            stack.push_back(new_node);
        }

        if desired_ingredients.contains(&6) {
            // Mouth Wash
            let mut mouth_wash_mix = Vec::new();

            for eff in &node.effs {
                match eff {
                    23 => inr(&mut mouth_wash_mix, 31), // Calming -> Anti-Gravity
                    4 => inr(&mut mouth_wash_mix, 11),  // Calorie-Dense -> Sneaky
                    35 => inr(&mut mouth_wash_mix, 2),  // Explosive -> Sedating
                    24 => inr(&mut mouth_wash_mix, 10), // Focused -> Jennerising
                    n => inr(&mut mouth_wash_mix, *n),  // Keep other effects unchanged
                };
            }

            inr(&mut mouth_wash_mix, 6); // Add the main effect (Mouth Wash)

            // Sort effects
            mouth_wash_mix.sort();
            let new_node = MixNode {
                effs: mouth_wash_mix,
                path: {
                    let mut path = node.path.clone();
                    path.push(6);
                    path
                },
            };

            if desired_effects
                .iter()
                .all(|&eff| new_node.effs.contains(&eff))
            {
                display_path(
                    &new_node.path,
                    &new_node.effs,
                    &mixer_map,
                    &effect_map,
                    &mut desired_path_count,
                );
            }

            stack.push_back(new_node);
        }

        if desired_ingredients.contains(&7) {
            // Motor Oil
            let mut motor_oil_mix = Vec::new();

            // Apply transformations
            for eff in &node.effs {
                match eff {
                    1 => inr(&mut motor_oil_mix, 18),  // Energizing -> Munchies
                    18 => inr(&mut motor_oil_mix, 33), // Munchies -> Schizo
                    14 => inr(&mut motor_oil_mix, 3),  // Foggy -> Toxic
                    17 => inr(&mut motor_oil_mix, 2),  // Euphoric -> Sedating
                    19 => inr(&mut motor_oil_mix, 31), // Paranoia -> Anti-Gravity
                    n => inr(&mut motor_oil_mix, *n),  // Keep other effects unchanged
                };
            }

            inr(&mut motor_oil_mix, 7); // Add the main effect (Motor Oil)

            // Sort effects
            motor_oil_mix.sort();
            let new_node = MixNode {
                effs: motor_oil_mix,
                path: {
                    let mut path = node.path.clone();
                    path.push(7);
                    path
                },
            };

            if desired_effects
                .iter()
                .all(|&eff| new_node.effs.contains(&eff))
            {
                display_path(
                    &new_node.path,
                    &new_node.effs,
                    &mixer_map,
                    &effect_map,
                    &mut desired_path_count,
                );
            }

            stack.push_back(new_node);
        }

        if desired_ingredients.contains(&8) {
            // Banana
            let mut banana_mix = Vec::new();

            // Apply transformations
            for eff in &node.effs {
                match eff {
                    1 => inr(&mut banana_mix, 15),  // energizing -> thought provoking
                    23 => inr(&mut banana_mix, 11), // Calming -> Sneaky
                    3 => inr(&mut banana_mix, 30),  // Toxic -> Smelly
                    34 => inr(&mut banana_mix, 25), // Long Faced -> Refreshing
                    21 => inr(&mut banana_mix, 15), // Cyclopean -> Thought-Provoking
                    29 => inr(&mut banana_mix, 24), // Disorienting -> Focused
                    24 => inr(&mut banana_mix, 32), // Focused -> Seizure-Inducing
                    19 => inr(&mut banana_mix, 10), // Paranoia -> Jennerising
                    30 => inr(&mut banana_mix, 31), // Smelly -> Anti-Gravity
                    n => inr(&mut banana_mix, *n),  // Keep other effects unchanged
                };
            }

            inr(&mut banana_mix, 8); // Add the main effect (Banana)

            // Sort effects
            banana_mix.sort();
            let new_node = MixNode {
                effs: banana_mix,
                path: {
                    let mut path = node.path.clone();
                    path.push(8);
                    path
                },
            };

            if desired_effects
                .iter()
                .all(|&eff| new_node.effs.contains(&eff))
            {
                display_path(
                    &new_node.path,
                    &new_node.effs,
                    &mixer_map,
                    &effect_map,
                    &mut desired_path_count,
                );
            }

            stack.push_back(new_node);
        }

        if desired_ingredients.contains(&9) {
            // Chili
            let mut chili_mix = Vec::new();

            for eff in &node.effs {
                match eff {
                    5 => inr(&mut chili_mix, 17),  // Athletic -> Euphoric
                    31 => inr(&mut chili_mix, 12), // Anti-Gravity -> Tropic Thunder
                    11 => inr(&mut chili_mix, 9),  // Sneaky -> Bright-Eyed
                    18 => inr(&mut chili_mix, 3),  // Munchies -> Toxic
                    22 => inr(&mut chili_mix, 34), // Laxative -> Long Faced
                    27 => inr(&mut chili_mix, 25), // Shrinking -> Refreshing
                    n => inr(&mut chili_mix, *n),  // Keep other effects unchanged
                };
            }

            inr(&mut chili_mix, 9); // Add the main effect (Chili)

            // Sort effects
            chili_mix.sort();
            let new_node = MixNode {
                effs: chili_mix,
                path: {
                    let mut path = node.path.clone();
                    path.push(9);
                    path
                },
            };

            if desired_effects
                .iter()
                .all(|&eff| new_node.effs.contains(&eff))
            {
                display_path(
                    &new_node.path,
                    &new_node.effs,
                    &mixer_map,
                    &effect_map,
                    &mut desired_path_count,
                );
            }

            stack.push_back(new_node);
        }

        if desired_ingredients.contains(&10) {
            // Iodine
            let mut iodine_mix = Vec::new();

            for eff in &node.effs {
                match eff {
                    23 => inr(&mut iodine_mix, 6),  // Calming -> Balding
                    3 => inr(&mut iodine_mix, 11),  // Toxic -> Sneaky
                    14 => inr(&mut iodine_mix, 19), // Foggy -> Paranoia
                    4 => inr(&mut iodine_mix, 8),   // Calorie-Dense -> Gingeritis
                    17 => inr(&mut iodine_mix, 32), // Euphoric -> Seizure-Inducing
                    25 => inr(&mut iodine_mix, 15), // Refreshing -> Thought-Provoking
                    n => inr(&mut iodine_mix, *n),  // Keep other effects unchanged
                };
            }

            inr(&mut iodine_mix, 10); // Add the main effect (Iodine)

            // Sort effects
            iodine_mix.sort();
            let new_node = MixNode {
                effs: iodine_mix,
                path: {
                    let mut path = node.path.clone();
                    path.push(10);
                    path
                },
            };

            if desired_effects
                .iter()
                .all(|&eff| new_node.effs.contains(&eff))
            {
                display_path(
                    &new_node.path,
                    &new_node.effs,
                    &mixer_map,
                    &effect_map,
                    &mut desired_path_count,
                );
            }

            stack.push_back(new_node);
        }

        if desired_ingredients.contains(&11) {
            // Paracetemol
            let mut paracetamol_mix = Vec::new();

            for eff in &node.effs {
                match eff {
                    1 => inr(&mut paracetamol_mix, 19),  // Energizing -> Paranoia
                    23 => inr(&mut paracetamol_mix, 7),  // Calming -> Slippery
                    3 => inr(&mut paracetamol_mix, 12),  // Toxic -> Tropic Thunder
                    9 => inr(&mut paracetamol_mix, 26),  // Spicy -> Bright-Eyed
                    28 => inr(&mut paracetamol_mix, 3),  // Glowing -> Toxic
                    14 => inr(&mut paracetamol_mix, 23), // Foggy -> Calming
                    18 => inr(&mut paracetamol_mix, 31), // Munchies -> Anti-Gravity
                    19 => inr(&mut paracetamol_mix, 6),  // Paranoia -> Balding
                    25 => inr(&mut paracetamol_mix, 5),  // Electrifying -> Athletic
                    24 => inr(&mut paracetamol_mix, 8),
                    n => inr(&mut paracetamol_mix, *n), // Keep other effects unchanged
                };
            }

            inr(&mut paracetamol_mix, 11); // Add the main effect (Paracetemol)

            // Sort effects
            paracetamol_mix.sort();
            let new_node = MixNode {
                effs: paracetamol_mix,
                path: {
                    let mut path = node.path.clone();
                    path.push(11);
                    path
                },
            };

            if desired_effects
                .iter()
                .all(|&eff| new_node.effs.contains(&eff))
            {
                display_path(
                    &new_node.path,
                    &new_node.effs,
                    &mixer_map,
                    &effect_map,
                    &mut desired_path_count,
                );
            }

            stack.push_back(new_node);
        }

        if desired_ingredients.contains(&12) {
            // Viagra
            let mut viagra_mix = Vec::new();

            for eff in &node.effs {
                match eff {
                    5 => inr(&mut viagra_mix, 11),  // Athletic -> Sneaky
                    17 => inr(&mut viagra_mix, 26), // Euphoric -> Bright-Eyed
                    22 => inr(&mut viagra_mix, 23), // Laxative -> Calming
                    29 => inr(&mut viagra_mix, 3),  // Disorienting -> Toxic
                    n => inr(&mut viagra_mix, *n),  // Keep other effects unchanged
                };
            }

            inr(&mut viagra_mix, 12); // Add the main effect (Viagra)

            // Sort effects
            viagra_mix.sort();
            let new_node = MixNode {
                effs: viagra_mix,
                path: {
                    let mut path = node.path.clone();
                    path.push(12);
                    path
                },
            };

            if desired_effects
                .iter()
                .all(|&eff| new_node.effs.contains(&eff))
            {
                display_path(
                    &new_node.path,
                    &new_node.effs,
                    &mixer_map,
                    &effect_map,
                    &mut desired_path_count,
                );
            }

            stack.push_back(new_node);
        }

        if desired_ingredients.contains(&13) {
            // Horse Semen
            let mut horse_semen_mix = Vec::new();

            for eff in &node.effs {
                match eff {
                    31 => inr(&mut horse_semen_mix, 23), // Anti-Gravity -> Calming
                    8 => inr(&mut horse_semen_mix, 25),  // Gingeritis -> Refreshing
                    15 => inr(&mut horse_semen_mix, 27), // Thought-Provoking -> Electrifying
                    n => inr(&mut horse_semen_mix, *n),  // Keep other effects unchanged
                };
            }

            inr(&mut horse_semen_mix, 13); // Add the main effect (Horse Semen)

            // Sort effects
            horse_semen_mix.sort();
            let new_node = MixNode {
                effs: horse_semen_mix,
                path: {
                    let mut path = node.path.clone();
                    path.push(13);
                    path
                },
            };

            if desired_effects
                .iter()
                .all(|&eff| new_node.effs.contains(&eff))
            {
                display_path(
                    &new_node.path,
                    &new_node.effs,
                    &mixer_map,
                    &effect_map,
                    &mut desired_path_count,
                );
            }

            stack.push_back(new_node);
        }

        if desired_ingredients.contains(&14) {
            // Mega Bean
            let mut mega_bean_mix = Vec::new();

            for eff in &node.effs {
                match eff {
                    1 => inr(&mut mega_bean_mix, 21), // Energizing -> Cyclopean
                    23 => inr(&mut mega_bean_mix, 28), // Calming -> Glowing
                    11 => {
                        inr(&mut mega_bean_mix, 23);
                        inr(&mut mega_bean_mix, 28);
                    }
                    10 => inr(&mut mega_bean_mix, 19), // Jennerising -> Paranoia
                    5 => inr(&mut mega_bean_mix, 22),  // Athletic -> Laxative
                    7 => inr(&mut mega_bean_mix, 2),   // Slippery -> Toxic
                    15 => {
                        inr(&mut mega_bean_mix, 1);
                        inr(&mut mega_bean_mix, 21);
                    }
                    34 => inr(&mut mega_bean_mix, 24), // Seizure-Inducing -> Focused
                    24 => inr(&mut mega_bean_mix, 29), // Focused -> Disorienting
                    27 => inr(&mut mega_bean_mix, 25), // Shrinking -> Electrifying
                    n => inr(&mut mega_bean_mix, *n),  // Keep other effects unchanged
                };
            }

            inr(&mut mega_bean_mix, 14); // Add the main effect (Mega Bean)

            // Sort effects
            mega_bean_mix.sort();
            let new_node = MixNode {
                effs: mega_bean_mix,
                path: {
                    let mut path = node.path.clone();
                    path.push(14);
                    path
                },
            };

            if desired_effects
                .iter()
                .all(|&eff| new_node.effs.contains(&eff))
            {
                display_path(
                    &new_node.path,
                    &new_node.effs,
                    &mixer_map,
                    &effect_map,
                    &mut desired_path_count,
                );
            }

            stack.push_back(new_node);
        }

        if desired_ingredients.contains(&15) {
            // Addy
            let mut addy_mix = Vec::new();

            for eff in &node.effs {
                match eff {
                    2 => inr(&mut addy_mix, 8),   // Sedating -> Gingeritis
                    32 => inr(&mut addy_mix, 25), // Long Faced -> Electrifying
                    28 => inr(&mut addy_mix, 26), // Glowing -> Refreshing
                    14 => inr(&mut addy_mix, 1),  // Foggy -> Energizing
                    35 => inr(&mut addy_mix, 17), // Explosive -> Euphoric
                    n => inr(&mut addy_mix, *n),  // Keep other effects unchanged
                };
            }

            inr(&mut addy_mix, 15); // Add the main effect (Addy)

            // Sort effects
            addy_mix.sort();
            let new_node = MixNode {
                effs: addy_mix,
                path: {
                    let mut path = node.path.clone();
                    path.push(15);
                    path
                },
            };

            if desired_effects
                .iter()
                .all(|&eff| new_node.effs.contains(&eff))
            {
                display_path(
                    &new_node.path,
                    &new_node.effs,
                    &mixer_map,
                    &effect_map,
                    &mut desired_path_count,
                );
            }

            stack.push_back(new_node);
        }

        if desired_ingredients.contains(&16) {
            // Battery
            let mut battery_mix = Vec::new();

            

            // Process effects
            for eff in &node.effs {
                match eff {
                    18 => inr(&mut battery_mix, 12), // Munchies -> Tropic Thunder
                    17  => inr(&mut battery_mix, 34), // Euphoric -> Zombifying
                    25  => inr(&mut battery_mix, 17), // Electrifying -> Euphoric
                    22 => inr(&mut battery_mix, 4),                     // Laxative -> Calorie-Dense
                    21 => inr(&mut battery_mix, 28),                    // Cyclopean -> Glowing
                    27 => inr(&mut battery_mix, 18),                    // Shrinking -> Munchies
                    n => inr(&mut battery_mix, *n), // Keep other effects unchanged
                };
            }

            inr(&mut battery_mix, 16); // Add the main effect (Battery)

            // Sort effects
            battery_mix.sort();
            let new_node = MixNode {
                effs: battery_mix,
                path: {
                    let mut path = node.path.clone();
                    path.push(16);
                    path
                },
            };

            if desired_effects
                .iter()
                .all(|&eff| new_node.effs.contains(&eff))
            {
                display_path(
                    &new_node.path,
                    &new_node.effs,
                    &mixer_map,
                    &effect_map,
                    &mut desired_path_count,
                );
            }

            stack.push_back(new_node);
        }
    }
}

// O(n)
fn inr<T>(vec: &mut Vec<T>, val: T)
where
    T: Eq + Ord,
{
    let mut i = 0;
    while let Some(k) = vec.get(i) {
        if *k == val {
            return;
        }
        i += 1;
    }
    vec.push(val);
    vec.sort();
}

fn display_path(
    path: &Vec<u8>,
    effs: &Vec<u8>,
    mixer_map: &HashMap<u8, &str>,
    effect_map: &HashMap<u8, &str>,
    desired_path_count: &mut i32,
) {
    let mut path_string = String::new();
    for (i, p) in path.iter().enumerate() {
        if i == path.len() - 1 {
            path_string.push_str(mixer_map.get(p).unwrap());
        } else {
            path_string.push_str(&format!("{} -> ", mixer_map.get(p).unwrap()));
        }
    }

    let mut effect_string = String::new();
    for (i, e) in effs.iter().enumerate() {
        if i == effs.len() - 1 {
            effect_string.push_str(effect_map.get(e).unwrap());
        } else {
            effect_string.push_str(&format!("{} + ", effect_map.get(e).unwrap()));
        }
    }

    println!("Path: {} | Effects: {}", path_string, effect_string);

    *desired_path_count -= 1;
    if *desired_path_count == 0 {
        panic!("Desired amount of paths printed, exiting.");
    }
}
