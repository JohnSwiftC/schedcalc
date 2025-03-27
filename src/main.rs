use std::{collections::{HashMap, HashSet, VecDeque}, hash::Hash};


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
        (9, "Spciy"),
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

    previous_nodes.insert(first_node.effs.clone());
    stack.push_back(first_node);

    while let Some(node) = stack.pop_front() {
        if previous_nodes.contains(&node.effs) {continue}

        if !node.path.ends_with(&[1]) { // Cuke
            let mut cuke_mix = Vec::new();
            for eff in &node.effs {
                match eff {
                    3 => inr(&mut cuke_mix, 17),
                    7 => inr(&mut cuke_mix, 18),
                    11 => inr(&mut cuke_mix, 19),
                    
                };
            }
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