use std::{collections::{HashMap, HashSet, VecDeque}};


struct MixNode {
    effs: Vec<u8>,
    path: Vec<u8>,
}

fn main() {
    let mut previous_nodes = HashSet::new(); // HashSet is sorted effects of a node.
    let mut stack = VecDeque::new();

    let first_node = MixNode {
        effs: vec![1],
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
        (35, "Explosive")
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
        if previous_nodes.contains(&node.effs) {continue;} else {previous_nodes.insert(node.effs.clone());}

        if !node.path.ends_with(&[1]) { // Cuke
            let mut cuke_mix = Vec::new();
            for eff in &node.effs {
                match eff {
                    3 => inr(&mut cuke_mix, 17),
                    7 => {inr(&mut cuke_mix, 18); inr(&mut cuke_mix, 5);}
                    11 => inr(&mut cuke_mix, 19),
                    20 => inr(&mut cuke_mix, 21),
                    8 => inr(&mut cuke_mix, 15),
                    18 => inr(&mut cuke_mix, 5),
                    17 => inr(&mut cuke_mix,22),
                    n => inr(&mut cuke_mix, *n),
                };
            }

            inr(&mut cuke_mix, 1);

            // We have now performed any swap, carried over unchanged effects, and added the main effect
            // Now check that this doesn't violate any rules.

            // Actually do this later


            // Sort effects

            cuke_mix.sort();
            stack.push_back(MixNode {
                effs: cuke_mix,
                path: {let mut path = node.path.clone(); path.push(1); path},
            });
        }

        if !node.path.ends_with(&[2]) { // Flu
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

            inr(&mut flu_mix, 2);

            // We have now performed any swap, carried over unchanged effects, and added the main effect
            // Now check that this doesn't violate any rules.

            // Actually do this later


            // Sort effects

            flu_mix.sort();
            stack.push_back(MixNode {
                effs: flu_mix,
                path: {let mut path = node.path.clone(); path.push(1); path},
            });
        }
    }


}


// O(n)
fn inr<T>(vec: &mut Vec<T>, val: T) where T: Eq {
    
    let mut i = 0;
    while let Some(k) = vec.get(i) {
        if *k == val {break}
        i += 1;
    }

    vec.push(val);
}