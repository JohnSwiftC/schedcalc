use std::collections::{HashMap, HashSet, VecDeque};

// Use a BitSet-like representation for effects
#[derive(Clone, PartialEq, Eq, Hash)]
struct EffectSet {
    // Using u64 can handle up to 64 effects (we only need 35)
    bits: u64,
}

impl EffectSet {
    fn new() -> Self {
        EffectSet { bits: 0 }
    }

    fn with_effect(effect: u8) -> Self {
        let mut set = EffectSet::new();
        set.add(effect);
        set
    }

    fn add(&mut self, effect: u8) {
        self.bits |= 1u64 << effect;
    }

    fn contains(&self, effect: u8) -> bool {
        (self.bits & (1u64 << effect)) != 0
    }

    fn count(&self) -> u32 {
        self.bits.count_ones()
    }

    // Convert to sorted Vec for display
    fn to_vec(&self) -> Vec<u8> {
        let mut result = Vec::with_capacity(self.count() as usize);
        for i in 1..=35 {
            if self.contains(i) {
                result.push(i);
            }
        }
        result
    }
}

#[derive(Clone)]
struct MixNode {
    effects: EffectSet,
    path: Vec<u8>,
}

// Data related to ingredients, effects and costs
struct MixData {
    effect_map: HashMap<u8, &'static str>,
    mixer_map: HashMap<u8, &'static str>,
    cost_map: HashMap<u8, i32>,
    multiplier_map: HashMap<u8, f32>,
}

impl MixData {
    fn new() -> Self {
        MixData {
            effect_map: HashMap::from([
                (1, "Energizing"), (2, "Sedating"), (3, "Toxic"), (4, "Calorie-Dense"), (5, "Athletic"),
                (6, "Balding"), (7, "Slippery"), (8, "Gingeritis"), (9, "Spicy"), (10, "Jennerising"),
                (11, "Sneaky"), (12, "Tropic Thunder"), (13, "Long Faced"), (14, "Foggy"), (15, "Thought-Provoking"),
                (16, "Bright Eyed"), (17, "Euphoric"), (18, "Munchies"), (19, "Paranoia"), (21, "Cyclopean"),
                (22, "Laxative"), (23, "Calming"), (24, "Focused"), (25, "Electrifying"), (26, "Refreshing"),
                (27, "Shrinking"), (28, "Glowing"), (29, "Disorienting"), (30, "Smelly"), (31, "Anti-Gravity"),
                (32, "Seizure Inducing"), (33, "Schizophrenia"), (34, "Zombifying"), (35, "Explosive"),
            ]),
            
            mixer_map: HashMap::from([
                (1, "Cuke"), (2, "Flu Medicine"), (3, "Gasoline"), (4, "Donut"), (5, "Energy Drink"),
                (6, "Mouth Wash"), (7, "Motor Oil"), (8, "Banana"), (9, "Chili"), (10, "Iodine"),
                (11, "Paracetemol"), (12, "Viagra"), (13, "Horse Semen"), (14, "Mega Bean"), (15, "Addy"),
                (16, "Battery"),
            ]),
            
            cost_map: HashMap::from([
                (1, 2), (2, 5), (3, 5), (4, 3), (5, 6), (6, 4), (7, 6), (8, 2),
                (9, 7), (10, 8), (11, 3), (12, 4), (13, 9), (14, 7), (15, 9), (16, 8),
            ]),
            
            multiplier_map: HashMap::from([
                (1, 0.22), (2, 0.26), (3, 0.0), (4, 0.28), (5, 0.32), (6, 0.30), (7, 0.34), (8, 0.2),
                (9, 0.38), (10, 0.42), (11, 0.24), (12, 0.46), (13, 0.52), (14, 0.36), (15, 0.44), (16, 0.40),
                (17, 0.18), (18, 0.12), (19, 0.0), (21, 0.56), (22, 0.0), (23, 0.1), (24, 0.16), (25, 0.50),
                (26, 0.14), (27, 0.6), (28, 0.48), (29, 0.0), (30, 0.0), (31, 0.54), (32, 0.0), (33, 0.0),
                (34, 0.58), (35, 0.0),
            ])
        }
    }
    
    // Calculate the multiplier for a set of effects
    fn calculate_multiplier(&self, effects: &EffectSet) -> f32 {
        let mut multiplier = 1.0; // Base multiplier
        for i in 1..=35 {
            if effects.contains(i) {
                if let Some(&m) = self.multiplier_map.get(&i) {
                    multiplier += m;
                }
            }
        }
        multiplier
    }
}

// Transformation maps for each ingredient
type TransformationMap = HashMap<u8, u8>;

fn create_transformation_maps() -> HashMap<u8, TransformationMap> {
    let mut maps = HashMap::new();
    
    // Cuke (1)
    maps.insert(1, HashMap::from([
        (3, 17),   // Toxic → Euphoric
        (7, 18),   // Slippery → Munchies
        (11, 19),  // Sneaky → Paranoia
        (14, 21),  // Foggy → Cyclopean
        (8, 15),   // Gingeritis → Thought-Provoking
        (18, 5),   // Munchies → Athletic
        (17, 22),  // Euphoric → Laxative
    ]));
    
    // Flu Medicine (2)
    maps.insert(2, HashMap::from([
        (23, 16),  // Calming → Bright Eyed
        (5, 18),   // Athletic → Munchies
        (15, 8),   // Thought-Provoking → Gingeritis
        (21, 14),  // Cyclopean → Foggy
        (18, 7),   // Munchies → Slippery
        (22, 17),  // Laxative → Euphoric
        (17, 3),   // Euphoric → Toxic
        (24, 23),  // Focused → Calming
        (25, 26),  // Electrifying → Refreshing
        (27, 19),  // Shrinking → Paranoia
    ]));
    
    // Gasoline (3)
    maps.insert(3, HashMap::from([
        (8, 30),   // Gingeritis → Smelly
        (10, 11),  // Jennerising → Sneaky
        (11, 12),  // Sneaky → Tropic Thunder
        (18, 2),   // Munchies → Sedating
        (1, 9),    // Energizing → Spicy
        (17, 1),   // Euphoric → Energizing
        (22, 14),  // Laxative → Foggy
        (29, 28),  // Disorienting → Glowing
        (19, 23),  // Paranoia → Calming
        (25, 29),  // Electrifying → Disorienting
        (27, 24),  // Shrinking → Focused
    ]));
    
    // Keep adding all transformation maps...
    // Donut (4)
    maps.insert(4, HashMap::from([
        (4, 35),   // Calorie-Dense → Explosive
        (6, 11),   // Balding → Sneaky
        (31, 7),   // Anti-Gravity → Slippery
        (10, 8),   // Jennerising → Gingeritis
        (24, 17),  // Focused → Euphoric
        (27, 1),   // Shrinking → Energizing
    ]));
    
    // Energy Drink (5)
    maps.insert(5, HashMap::from([
        (2, 18),   // Sedating → Munchies
        (17, 1),   // Euphoric → Energizing
        (9, 17),   // Spicy → Euphoric
        (12, 11),  // Tropic Thunder → Sneaky
        (28, 29),  // Glowing → Disorienting
        (14, 22),  // Foggy → Laxative
        (29, 25),  // Disorienting → Electrifying
        (33, 6),   // Schizophrenia → Balding
        (24, 27),  // Focused → Shrinking
    ]));
    
    // Mouth Wash (6)
    maps.insert(6, HashMap::from([
        (23, 31),  // Calming → Anti-Gravity
        (4, 11),   // Calorie-Dense → Sneaky
        (35, 2),   // Explosive → Sedating
        (24, 10),  // Focused → Jennerising
    ]));
    
    // Motor Oil (7)
    maps.insert(7, HashMap::from([
        (1, 18),   // Energizing → Munchies
        (18, 33),  // Munchies → Schizophrenia
        (14, 3),   // Foggy → Toxic
        (17, 2),   // Euphoric → Sedating
        (19, 31),  // Paranoia → Anti-Gravity
    ]));
    
    // Banana (8)
    maps.insert(8, HashMap::from([
        (1, 15),   // Energizing → Thought-Provoking
        (23, 11),  // Calming → Sneaky
        (3, 30),   // Toxic → Smelly
        (13, 26),  // Long Faced → Refreshing
        (21, 15),  // Cyclopean → Thought-Provoking
        (29, 24),  // Disorienting → Focused
        (24, 32),  // Focused → Seizure-Inducing
        (19, 10),  // Paranoia → Jennerising
        (30, 31),  // Smelly → Anti-Gravity
    ]));
    
    // Chili (9)
    maps.insert(9, HashMap::from([
        (5, 17),   // Athletic → Euphoric
        (31, 12),  // Anti-Gravity → Tropic Thunder
        (11, 16),  // Sneaky → Bright-Eyed
        (18, 3),   // Munchies → Toxic
        (22, 13),  // Laxative → Long Faced
        (27, 26),  // Shrinking → Refreshing
    ]));
    
    // Iodine (10)
    maps.insert(10, HashMap::from([
        (23, 6),   // Calming → Balding
        (3, 11),   // Toxic → Sneaky
        (14, 19),  // Foggy → Paranoia
        (4, 8),    // Calorie-Dense → Gingeritis
        (17, 32),  // Euphoric → Seizure-Inducing
        (26, 15),  // Refreshing → Thought-Provoking
    ]));
    
    // Paracetemol (11)
    maps.insert(11, HashMap::from([
        (1, 19),   // Energizing → Paranoia
        (23, 7),   // Calming → Slippery
        (3, 12),   // Toxic → Tropic Thunder
        (9, 16),   // Spicy → Bright-Eyed
        (28, 3),   // Glowing → Toxic
        (14, 23),  // Foggy → Calming
        (18, 31),  // Munchies → Anti-Gravity
        (19, 6),   // Paranoia → Balding
        (25, 5),   // Electrifying → Athletic
        (24, 8),   // Focused → Gingeritis
    ]));
    
    // Viagra (12)
    maps.insert(12, HashMap::from([
        (5, 11),   // Athletic → Sneaky
        (17, 16),  // Euphoric → Bright-Eyed
        (22, 23),  // Laxative → Calming
        (29, 3),   // Disorienting → Toxic
    ]));
    
    // Horse Semen (13)
    maps.insert(13, HashMap::from([
        (31, 23),  // Anti-Gravity → Calming
        (8, 26),   // Gingeritis → Refreshing
        (15, 25),  // Thought-Provoking → Electrifying
    ]));
    
    // Mega Bean (14)
    maps.insert(14, HashMap::from([
        (1, 21),   // Energizing → Cyclopean
        (23, 28),  // Calming → Glowing
        (11, 23),  // Sneaky → Calming
        (10, 19),  // Jennerising → Paranoia
        (5, 22),   // Athletic → Laxative
        (7, 3),    // Slippery → Toxic
        (15, 1),   // Thought-Provoking → Energizing
        (32, 24),  // Seizure-Inducing → Focused
        (24, 29),  // Focused → Disorienting
        (27, 25),  // Shrinking → Electrifying
    ]));
    
    // Addy (15)
    maps.insert(15, HashMap::from([
        (2, 8),    // Sedating → Gingeritis
        (13, 25),  // Long Faced → Electrifying
        (28, 26),  // Glowing → Refreshing
        (14, 1),   // Foggy → Energizing
        (35, 17),  // Explosive → Euphoric
    ]));
    
    // Battery (16)
    maps.insert(16, HashMap::from([
        (18, 12),  // Munchies → Tropic Thunder
        (17, 34),  // Euphoric → Zombifying
        (25, 17),  // Electrifying → Euphoric
        (22, 4),   // Laxative → Calorie-Dense
        (21, 28),  // Cyclopean → Glowing
        (27, 18),  // Shrinking → Munchies
    ]));
    
    maps
}

pub fn run(desired_ingredients: HashSet<u8>, starting_effect: Option<u8>, max: &mut f32) {
    let transformation_maps = create_transformation_maps();
    let mix_data = MixData::new();
    
    // Initialize first node
    let first_node = MixNode {
        effects: if let Some(n) = starting_effect {
            EffectSet::with_effect(n)
        } else {
            EffectSet::new()
        },
        path: Vec::new(),
    };
    
    let mut previous_nodes = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back(first_node);
    
    // Track the best node so far
    let mut best_node: Option<MixNode> = None;
    
    while let Some(node) = queue.pop_front() {
        // Skip if we've seen this effect combination before
        if previous_nodes.contains(&node.effects) {
            continue;
        }
        
        // Skip if we have too many effects
        if node.effects.count() > 8 {
            continue;
        }
        
        // Mark this node as visited
        previous_nodes.insert(node.effects.clone());
        
        // Calculate current multiplier
        let current_multiplier = mix_data.calculate_multiplier(&node.effects);
        
        // Update best node if this one has a higher multiplier
        if current_multiplier > *max {
            *max = current_multiplier;
            best_node = Some(node.clone());
            display_path(&node.path, &node.effects, &mix_data);
        }
        
        // Try each desired ingredient
        for &ingredient in &desired_ingredients {
            let new_node = apply_ingredient(&node, ingredient, &transformation_maps);
            queue.push_back(new_node);
        }
    }
}

fn apply_ingredient(node: &MixNode, ingredient: u8, transformation_maps: &HashMap<u8, TransformationMap>) -> MixNode {
    let transform_map = transformation_maps.get(&ingredient).unwrap();
    let mut new_effects = EffectSet::new();
    let mut munchies_transformed = false; // Track if Munchies (18) has been transformed
    
    // Process transformations following original logic
    for i in 1..=35 {
        if node.effects.contains(i) {
            // Special handling for Munchies effect (18)
            if i == 18 && munchies_transformed {
                continue; // Skip if Munchies already transformed once
            }
            
            if let Some(&transformed) = transform_map.get(&i) {
                new_effects.add(transformed);
                if i == 18 {
                    munchies_transformed = true;
                }
            } else {
                new_effects.add(i);
            }
        }
    }
    
    // Add the ingredient's effect if we have space
    if new_effects.count() < 8 {
        new_effects.add(ingredient);
    }
    
    // Create new path
    let mut new_path = node.path.clone();
    new_path.push(ingredient);
    
    MixNode {
        effects: new_effects,
        path: new_path,
    }
}

fn display_path(path: &Vec<u8>, effects: &EffectSet, mix_data: &MixData) {
    let effects_vec = effects.to_vec();
    
    // Calculate cost
    let mut cost = 0;
    for &p in path {
        cost += mix_data.cost_map.get(&p).unwrap_or(&0);
    }
    
    // Calculate multiplier
    let mut multi: f32 = 0.0;
    for &e in &effects_vec {
        multi += mix_data.multiplier_map.get(&e).unwrap_or(&0.0);
    }
    
    // Build path string
    let mut path_string = String::new();
    for (i, &p) in path.iter().enumerate() {
        if i == path.len() - 1 {
            path_string.push_str(mix_data.mixer_map.get(&p).unwrap_or(&"Unknown"));
        } else {
            path_string.push_str(&format!("{} -> ", mix_data.mixer_map.get(&p).unwrap_or(&"Unknown")));
        }
    }
    
    // Build effect string
    let mut effect_string = String::new();
    for (i, &e) in effects_vec.iter().enumerate() {
        if i == effects_vec.len() - 1 {
            effect_string.push_str(mix_data.effect_map.get(&e).unwrap_or(&"Unknown"));
        } else {
            effect_string.push_str(&format!("{} + ", mix_data.effect_map.get(&e).unwrap_or(&"Unknown")));
        }
    }
    
    println!(
        "Path: {} | Effects: {} | Cost: {} | Sell Multiplier: {}", 
        path_string, effect_string, cost, multi + 1.0
    );
}
