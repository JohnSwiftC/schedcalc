use std::collections::{HashMap, HashSet, VecDeque};

struct MixNode {
    effs: Vec<u8>,
    path: Vec<u8>,
}

fn main() {
    let mut desired_effects: Vec<u8> = vec![34, 25, 31, 35, 10, 32];
    desired_effects.sort();

    let mut previous_nodes = HashSet::new(); // HashSet is sorted effects of a node.
    let mut stack = VecDeque::new();

    let first_node = MixNode {
        effs: vec![23],
        path: Vec::new(),
    };

    // Encodings

    let mut effect_map = HashMap::from([
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
        (15, "Though-Provoking"),
        (16, "Bright Eyed"),
        (17, "Euphoric"),
        (18, "Munchies"),
        (19, "Paranoia"),
        (20, "Foggy"),
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

    let mut mixer_map = HashMap::from([
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

        if !node.path.ends_with(&[1]) {
            // Cuke
            let mut cuke_mix = Vec::new();
            for eff in &node.effs {
                match eff {
                    3 => inr(&mut cuke_mix, 17), // Toxic → Euphoric
                    7 => {
                        if contains_munchies {
                            inr(&mut cuke_mix, 5); // Slippery + Munchies = Athletic
                            munchies_mixed = true;
                        } else {
                            inr(&mut cuke_mix, 18); // Slippery → Munchies
                        }
                    }
                    11 => inr(&mut cuke_mix, 19), // Sneaky → Paranoia
                    20 => inr(&mut cuke_mix, 21), // Foggy → Cyclopean
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
                println!(
                    "Match found! Path: {:?}, Effects: {:?}",
                    new_node.path, new_node.effs
                );
            }

            stack.push_back(new_node);
        }

        if !node.path.ends_with(&[2]) {
            // Flu
            let mut flu_mix = Vec::new();
            for eff in &node.effs {
                match eff {
                    23 => inr(&mut flu_mix, 16),
                    5 => inr(&mut flu_mix, 18),
                    15 => inr(&mut flu_mix, 8),
                    21 => inr(&mut flu_mix, 20),
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
                println!(
                    "Match found! Path: {:?}, Effects: {:?}",
                    new_node.path, new_node.effs
                );
            }

            stack.push_back(new_node);
        }

        if !node.path.ends_with(&[3]) {
            // Gasoline // RULE CHANGE! If energizing and euphoric are present, it will turn energizing into spicy and leave euphoric
            let mut gas_mix = Vec::new();
            for eff in &node.effs {
                match eff {
                    1 => {
                        inr(&mut gas_mix, 17); // Energizing → Euphoric
                    }
                    8 => inr(&mut gas_mix, 30),  // Gingeritis → Smelly
                    10 => inr(&mut gas_mix, 11), // Jennerising → Sneaky
                    11 => inr(&mut gas_mix, 12), // Sneaky → Tropic Thunder
                    18 => inr(&mut gas_mix, 2),  // Munchies → Sedating
                    17 => {
                        if !node.effs.contains(&1) {
                            inr(&mut gas_mix, 9); // Euphoric → Spicy (Only if Energizing not present)
                        }
                    }
                    22 => inr(&mut gas_mix, 20), // Laxative → Foggy
                    29 => inr(&mut gas_mix, 28), // Disorienting → Glowing
                    19 => inr(&mut gas_mix, 23), // Paranoia → Calming
                    25 => inr(&mut gas_mix, 29), // Electrifying → Disorienting
                    27 => inr(&mut gas_mix, 24), // Shrinking → Focused
                    n => inr(&mut gas_mix, *n),  // Keep other effects unchanged
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
                println!(
                    "Match found! Path: {:?}, Effects: {:?}",
                    new_node.path, new_node.effs
                );
            }

            stack.push_back(new_node);
        }

        if !node.path.ends_with(&[4]) {
            // Donut
            let mut donut_mix = Vec::new();
            let has_calorie_dense = node.effs.contains(&4);
            let has_explosive = node.effs.contains(&35);

            for eff in &node.effs {
                match eff {
                    6 => inr(&mut donut_mix, 19),  // Balding -> Sneaky
                    31 => inr(&mut donut_mix, 7),  // Anti-Gravity -> Slippery
                    10 => inr(&mut donut_mix, 8),  // Jennerising -> Gingeritis
                    24 => inr(&mut donut_mix, 17), // Focused -> Euphoric
                    27 => inr(&mut donut_mix, 1),  // Shrinking -> Energizing
                    n => inr(&mut donut_mix, *n),  // Keep other effects unchanged
                };
            }

            // Add Explosive (35) if Calorie-Dense (4) is present and Explosive is not
            if has_calorie_dense && !has_explosive {
                inr(&mut donut_mix, 35);
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
                println!(
                    "Match found! Path: {:?}, Effects: {:?}",
                    new_node.path, new_node.effs
                );
            }

            stack.push_back(new_node);
        }

        if !node.path.ends_with(&[5]) {
            // Energy Drink
            let mut energy_drink_mix = Vec::new();

            for eff in &node.effs {
                match eff {
                    2 => inr(&mut energy_drink_mix, 18),  // Sedating -> Munchies
                    17 => inr(&mut energy_drink_mix, 1),  // Euphoric -> Energizing
                    9 => inr(&mut energy_drink_mix, 17),  // Spicy -> Euphoric
                    12 => inr(&mut energy_drink_mix, 11), // Tropic Thunder -> Sneaky
                    28 => inr(&mut energy_drink_mix, 29), // Glowing -> Disorienting
                    20 => inr(&mut energy_drink_mix, 22), // Foggy -> Laxative
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
                println!(
                    "Match found! Path: {:?}, Effects: {:?}",
                    new_node.path, new_node.effs
                );
            }

            stack.push_back(new_node);
        }

        if !node.path.ends_with(&[6]) {
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
                println!(
                    "Match found! Path: {:?}, Effects: {:?}",
                    new_node.path, new_node.effs
                );
            }

            stack.push_back(new_node);
        }

        if !node.path.ends_with(&[7]) {
            // Motor Oil
            let mut motor_oil_mix = Vec::new();
            let mut has_munchies = false;

            // First pass to check for Energizing
            for eff in &node.effs {
                if *eff == 18 {
                    has_munchies = true;
                }
            }

            // Apply transformations
            for eff in &node.effs {
                match eff {
                    1 => {
                        inr(&mut motor_oil_mix, 18); // Energizing -> Munchies
                        if has_munchies {
                            inr(&mut motor_oil_mix, 33); // Energizing -> Schizophrenia
                        } else {
                            inr(&mut motor_oil_mix, 18); // Energizing -> Munchies
                        }
                    }
                    20 => inr(&mut motor_oil_mix, 3), // Foggy -> Toxic
                    17 => inr(&mut motor_oil_mix, 2), // Euphoric -> Sedating
                    19 => inr(&mut motor_oil_mix, 31), // Paranoia -> Anti-Gravity
                    n => inr(&mut motor_oil_mix, *n), // Keep other effects unchanged
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
                println!(
                    "Match found! Path: {:?}, Effects: {:?}",
                    new_node.path, new_node.effs
                );
            }

            stack.push_back(new_node);
        }

        if !node.path.ends_with(&[8]) {
            // Banana
            let mut banana_mix = Vec::new();
            let mut has_cyclopean = false;

            // First pass to check for Cyclopean
            for eff in &node.effs {
                if *eff == 21 {
                    has_cyclopean = true;
                }
            }

            // Apply transformations
            for eff in &node.effs {
                match eff {
                    1 if !has_cyclopean => inr(&mut banana_mix, 15), // Energizing -> Thought-Provoking (if no Cyclopean)
                    23 => inr(&mut banana_mix, 11),                  // Calming -> Sneaky
                    3 => inr(&mut banana_mix, 30),                   // Toxic -> Smelly
                    34 => inr(&mut banana_mix, 25),                  // Long Faced -> Refreshing
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
                println!(
                    "Match found! Path: {:?}, Effects: {:?}",
                    new_node.path, new_node.effs
                );
            }

            stack.push_back(new_node);
        }

        if !node.path.ends_with(&[9]) {
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
                println!(
                    "Match found! Path: {:?}, Effects: {:?}",
                    new_node.path, new_node.effs
                );
            }

            stack.push_back(new_node);
        }

        if !node.path.ends_with(&[10]) {
            // Iodine
            let mut iodine_mix = Vec::new();

            for eff in &node.effs {
                match eff {
                    23 => inr(&mut iodine_mix, 6),  // Calming -> Balding
                    3 => inr(&mut iodine_mix, 11),  // Toxic -> Sneaky
                    20 => inr(&mut iodine_mix, 19), // Foggy -> Paranoia
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
                println!(
                    "Match found! Path: {:?}, Effects: {:?}",
                    new_node.path, new_node.effs
                );
            }

            stack.push_back(new_node);
        }

        if !node.path.ends_with(&[11]) {
            // Paracetemol
            let mut paracetamol_mix = Vec::new();
            let has_munchies = node.effs.contains(&18); // Check for Munchies presence
            let has_paranoia = node.effs.contains(&19); // Check for Paranoia presence

            for eff in &node.effs {
                match eff {
                    1 if !has_munchies => inr(&mut paracetamol_mix, 19), // Energizing -> Paranoia (if no Munchies)
                    23 => inr(&mut paracetamol_mix, 7),                  // Calming -> Slippery
                    3 => inr(&mut paracetamol_mix, 12),                  // Toxic -> Tropic Thunder
                    9 => inr(&mut paracetamol_mix, 26),                  // Spicy -> Bright-Eyed
                    28 => inr(&mut paracetamol_mix, 3),                  // Glowing -> Toxic
                    20 => inr(&mut paracetamol_mix, 23),                 // Foggy -> Calming
                    18 => inr(&mut paracetamol_mix, 31),                 // Munchies -> Anti-Gravity
                    1 if has_paranoia => inr(&mut paracetamol_mix, 6), // Energizing -> Balding (if Paranoia present)
                    25 => inr(&mut paracetamol_mix, 5),                // Electrifying -> Athletic
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
                println!(
                    "Match found! Path: {:?}, Effects: {:?}",
                    new_node.path, new_node.effs
                );
            }

            stack.push_back(new_node);
        }

        if !node.path.ends_with(&[12]) {
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
                println!(
                    "Match found! Path: {:?}, Effects: {:?}",
                    new_node.path, new_node.effs
                );
            }

            stack.push_back(new_node);
        }

        if !node.path.ends_with(&[13]) {
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
                println!(
                    "Match found! Path: {:?}, Effects: {:?}",
                    new_node.path, new_node.effs
                );
            }

            stack.push_back(new_node);
        }

        if !node.path.ends_with(&[14]) {
            // Mega Bean
            let mut mega_bean_mix = Vec::new();
            let has_thought_provoking = node.effs.contains(&15);

            for eff in &node.effs {
                match eff {
                    1 if !has_thought_provoking => inr(&mut mega_bean_mix, 21), // Energizing -> Cyclopean (unless Thought-Provoking is present)
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
                println!(
                    "Match found! Path: {:?}, Effects: {:?}",
                    new_node.path, new_node.effs
                );
            }

            stack.push_back(new_node);
        }

        if !node.path.ends_with(&[15]) {
            // Addy
            let mut addy_mix = Vec::new();

            for eff in &node.effs {
                match eff {
                    2 => inr(&mut addy_mix, 8),   // Sedating -> Gingeritis
                    32 => inr(&mut addy_mix, 25), // Long Faced -> Electrifying
                    28 => inr(&mut addy_mix, 26), // Glowing -> Refreshing
                    20 => inr(&mut addy_mix, 1),  // Foggy -> Energizing
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
                println!(
                    "Match found! Path: {:?}, Effects: {:?}",
                    new_node.path, new_node.effs
                );
            }

            stack.push_back(new_node);
        }

        if !node.path.ends_with(&[16]) {
            // Battery
            let mut battery_mix = Vec::new();
            let mut has_electrifying = false;
            let mut has_zombifying = false;

            // First, scan for Electrifying and Zombifying presence
            for eff in &node.effs {
                if *eff == 25 {
                    has_electrifying = true;
                }
                if *eff == 34 {
                    has_zombifying = true;
                }
            }

            // Process effects
            for eff in &node.effs {
                match eff {
                    18 => inr(&mut battery_mix, 12), // Munchies -> Tropic Thunder
                    17 if !has_electrifying => inr(&mut battery_mix, 34), // Euphoric -> Zombifying (if Electrifying not present)
                    25 if !has_zombifying => inr(&mut battery_mix, 17), // Electrifying -> Euphoric (if Zombifying not present)
                    22 => inr(&mut battery_mix, 4),                     // Laxative -> Calorie-Dense
                    25 => inr(&mut battery_mix, 17),                    // Electrifying -> Euphoric
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
                println!(
                    "Match found! Path: {:?}, Effects: {:?}",
                    new_node.path, new_node.effs
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
            break;
        }
        i += 1;
    }
    vec.push(val);
    vec.sort();
}
