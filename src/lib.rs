use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ---------------------------------------------------------------------------
// PetSpecies
// ---------------------------------------------------------------------------

/// Available pet species that a player can adopt.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PetSpecies {
    Dragon,
    Phoenix,
    Wolf,
    Owl,
    Fox,
    Cat,
    Turtle,
    Eagle,
    Unicorn,
    Robot,
}

impl PetSpecies {
    /// Human-readable name of the species.
    pub fn name(&self) -> &'static str {
        match self {
            Self::Dragon => "Dragon",
            Self::Phoenix => "Phoenix",
            Self::Wolf => "Wolf",
            Self::Owl => "Owl",
            Self::Fox => "Fox",
            Self::Cat => "Cat",
            Self::Turtle => "Turtle",
            Self::Eagle => "Eagle",
            Self::Unicorn => "Unicorn",
            Self::Robot => "Robot",
        }
    }
}

// ---------------------------------------------------------------------------
// PetStats
// ---------------------------------------------------------------------------

/// Core stats for a pet — all values live in [0, 1] except `experience` and `level`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PetStats {
    pub hunger: f64,
    pub happiness: f64,
    pub experience: f64,
    pub level: u32,
    pub bond: f64,
}

impl PetStats {
    /// Create a fresh set of stats for a newly-adopted pet.
    pub fn new() -> Self {
        Self {
            hunger: 0.5,
            happiness: 0.8,
            experience: 0.0,
            level: 1,
            bond: 0.3,
        }
    }

    /// Feed the pet — reduces hunger and slightly increases bond.
    pub fn feed(&mut self) {
        self.hunger = (self.hunger - 0.3).max(0.0);
        self.bond = (self.bond + 0.02).min(1.0);
    }

    /// Play with the pet — boosts happiness and slightly increases bond.
    pub fn play(&mut self) {
        self.happiness = (self.happiness + 0.25).min(1.0);
        self.bond = (self.bond + 0.03).min(1.0);
    }

    /// Train the pet — awards XP and slightly increases bond.
    pub fn train(&mut self, xp: f64) {
        self.experience += xp;
        self.bond = (self.bond + 0.01).min(1.0);
    }

    /// Simulate time passing — stats decay naturally.
    pub fn tick(&mut self) {
        self.hunger = (self.hunger + 0.05).min(1.0);
        self.happiness = (self.happiness - 0.03).max(0.0);
        if self.hunger > 0.8 {
            self.bond = (self.bond - 0.01).max(0.0);
        }
    }

    /// Attempt to level up. Returns `true` if the pet gained a level.
    /// Threshold is `level * 100` XP.
    pub fn level_up(&mut self) -> bool {
        let threshold = f64::from(self.level) * 100.0;
        if self.experience >= threshold {
            self.experience -= threshold;
            self.level += 1;
            self.happiness = (self.happiness + 0.1).min(1.0);
            self.bond = (self.bond + 0.05).min(1.0);
            true
        } else {
            false
        }
    }
}

impl Default for PetStats {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// PetMood
// ---------------------------------------------------------------------------

/// The current mood of a pet, derived from its stats.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PetMood {
    Happy,
    Hungry,
    Sleepy,
    Excited,
    Curious,
    Loyal,
}

impl PetMood {
    /// Determine mood from the current stats.
    pub fn from_stats(stats: &PetStats) -> Self {
        if stats.hunger > 0.8 {
            Self::Hungry
        } else if stats.happiness > 0.8 && stats.bond > 0.7 {
            Self::Loyal
        } else if stats.happiness > 0.7 {
            Self::Happy
        } else if stats.experience > 50.0 {
            Self::Excited
        } else if stats.bond > 0.5 {
            Self::Curious
        } else {
            Self::Sleepy
        }
    }
}

// ---------------------------------------------------------------------------
// Pet
// ---------------------------------------------------------------------------

/// A player's companion pet.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Pet {
    pub name: String,
    pub species: PetSpecies,
    pub stats: PetStats,
    pub tricks: Vec<String>,
    pub accessories: Vec<String>,
}

impl Pet {
    /// Create a brand-new pet.
    pub fn new(name: &str, species: PetSpecies) -> Self {
        Self {
            name: name.to_string(),
            species,
            stats: PetStats::new(),
            tricks: Vec::new(),
            accessories: Vec::new(),
        }
    }

    /// Teach the pet a new trick (no duplicates).
    pub fn teach_trick(&mut self, trick: &str) {
        if !self.tricks.contains(&trick.to_string()) {
            self.tricks.push(trick.to_string());
            self.stats.train(10.0);
        }
    }

    /// Add an accessory to the pet.
    pub fn add_accessory(&mut self, item: &str) {
        if !self.accessories.contains(&item.to_string()) {
            self.accessories.push(item.to_string());
        }
    }

    /// Generate a prose description of the pet's current appearance.
    pub fn appearance(&self) -> String {
        let mood = PetMood::from_stats(&self.stats);
        let mood_str = match mood {
            PetMood::Happy => "happily wagging its tail",
            PetMood::Hungry => "looking around for food",
            PetMood::Sleepy => "drowsily blinking its eyes",
            PetMood::Excited => "bouncing with energy",
            PetMood::Curious => "tilting its head inquisitively",
            PetMood::Loyal => "gazing at you devotedly",
        };

        let size = match self.stats.level {
            1..=3 => "small",
            4..=6 => "medium-sized",
            7..=9 => "large",
            _ => "magnificent",
        };

        let mut desc = format!(
            "{} the {} {} is {}.",
            self.name,
            size,
            self.species.name(),
            mood_str,
        );

        if !self.accessories.is_empty() {
            desc.push_str(&format!(
                " Wearing {}.",
                self.accessories.join(", ")
            ));
        }

        if !self.tricks.is_empty() {
            desc.push_str(&format!(
                " Knows tricks: {}.",
                self.tricks.join(", ")
            ));
        }

        desc
    }
}

// ---------------------------------------------------------------------------
// PetEvolution
// ---------------------------------------------------------------------------

/// Defines a possible evolution from one species into another.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PetEvolution {
    pub from_species: PetSpecies,
    pub to_species: PetSpecies,
    pub min_level: u32,
    pub min_bond: f64,
}

impl PetEvolution {
    /// Check whether the given pet satisfies the requirements for this evolution.
    pub fn can_evolve(&self, pet: &Pet) -> bool {
        pet.species == self.from_species
            && pet.stats.level >= self.min_level
            && pet.stats.bond >= self.min_bond
    }

    /// Perform the evolution in-place. Returns `false` if requirements aren't met.
    pub fn evolve(&self, pet: &mut Pet) -> bool {
        if !self.can_evolve(pet) {
            return false;
        }
        pet.species = self.to_species.clone();
        pet.stats.happiness = 1.0;
        pet.stats.bond = (pet.stats.bond + 0.1).min(1.0);
        true
    }

    /// The built-in evolution table.
    pub fn all_evolutions() -> Vec<Self> {
        vec![
            Self { from_species: PetSpecies::Cat, to_species: PetSpecies::Unicorn, min_level: 5, min_bond: 0.5 },
            Self { from_species: PetSpecies::Turtle, to_species: PetSpecies::Dragon, min_level: 8, min_bond: 0.6 },
            Self { from_species: PetSpecies::Fox, to_species: PetSpecies::Phoenix, min_level: 10, min_bond: 0.7 },
            Self { from_species: PetSpecies::Wolf, to_species: PetSpecies::Unicorn, min_level: 7, min_bond: 0.6 },
            Self { from_species: PetSpecies::Robot, to_species: PetSpecies::Eagle, min_level: 12, min_bond: 0.8 },
        ]
    }
}

// ---------------------------------------------------------------------------
// PetManager
// ---------------------------------------------------------------------------

/// Manages all pets across all players.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PetManager {
    pets: HashMap<String, Pet>,
}

impl PetManager {
    pub fn new() -> Self {
        Self {
            pets: HashMap::new(),
        }
    }

    /// Adopt a new pet for a player. Does nothing if the player already has one.
    pub fn adopt(&mut self, player: &str, name: &str, species: PetSpecies) {
        self.pets.entry(player.to_string()).or_insert_with(|| Pet::new(name, species));
    }

    /// Get a reference to a player's pet.
    pub fn get_pet(&self, player: &str) -> Option<&Pet> {
        self.pets.get(player)
    }

    /// Get a mutable reference to a player's pet.
    pub fn get_pet_mut(&mut self, player: &str) -> Option<&mut Pet> {
        self.pets.get_mut(player)
    }

    /// Check if a player's pet can evolve.
    pub fn can_evolve_pet(&self, player: &str) -> bool {
        let Some(pet) = self.pets.get(player) else {
            return false;
        };
        PetEvolution::all_evolutions()
            .iter()
            .any(|evo| evo.can_evolve(pet))
    }

    /// Evolve a player's pet if possible. Returns `true` on success.
    pub fn evolve_pet(&mut self, player: &str) -> bool {
        let pet = match self.pets.get(player) {
            Some(p) => p.clone(),
            None => return false,
        };
        for evo in PetEvolution::all_evolutions() {
            if evo.can_evolve(&pet) {
                // SAFETY: we just checked can_evolve
                if let Some(pet_mut) = self.pets.get_mut(player) {
                    return evo.evolve(pet_mut);
                }
            }
        }
        false
    }

    /// List all players that have pets.
    pub fn players(&self) -> Vec<&str> {
        self.pets.keys().map(|s| s.as_str()).collect()
    }
}

impl Default for PetManager {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // -- PetSpecies --
    #[test]
    fn species_name() {
        assert_eq!(PetSpecies::Dragon.name(), "Dragon");
        assert_eq!(PetSpecies::Robot.name(), "Robot");
    }

    #[test]
    fn species_serde_roundtrip() {
        let sp = PetSpecies::Fox;
        let json = serde_json::to_string(&sp).unwrap();
        let back: PetSpecies = serde_json::from_str(&json).unwrap();
        assert_eq!(sp, back);
    }

    // -- PetStats --
    #[test]
    fn stats_new_defaults() {
        let s = PetStats::new();
        assert_eq!(s.level, 1);
        assert_eq!(s.experience, 0.0);
        assert!(s.hunger > 0.0 && s.hunger < 1.0);
        assert!(s.happiness > 0.0 && s.happiness <= 1.0);
        assert!(s.bond > 0.0 && s.bond <= 1.0);
    }

    #[test]
    fn feed_reduces_hunger() {
        let mut s = PetStats::new();
        let before = s.hunger;
        s.feed();
        assert!(s.hunger < before);
    }

    #[test]
    fn play_increases_happiness() {
        let mut s = PetStats::new();
        s.happiness = 0.5;
        s.play();
        assert!(s.happiness > 0.5);
    }

    #[test]
    fn train_adds_xp() {
        let mut s = PetStats::new();
        s.train(42.0);
        assert_eq!(s.experience, 42.0);
    }

    #[test]
    fn tick_decays_stats() {
        let mut s = PetStats { hunger: 0.0, happiness: 1.0, experience: 0.0, level: 1, bond: 0.5 };
        s.tick();
        assert!(s.hunger > 0.0);
        assert!(s.happiness < 1.0);
    }

    #[test]
    fn tick_hunger_reduces_bond() {
        let mut s = PetStats { hunger: 0.9, happiness: 0.5, experience: 0.0, level: 1, bond: 0.5 };
        s.tick();
        assert!(s.bond < 0.5);
    }

    #[test]
    fn level_up_succeeds() {
        let mut s = PetStats { hunger: 0.0, happiness: 0.5, experience: 150.0, level: 1, bond: 0.5 };
        assert!(s.level_up());
        assert_eq!(s.level, 2);
        assert_eq!(s.experience, 50.0); // 150 - (1*100)
    }

    #[test]
    fn level_up_fails_insufficient_xp() {
        let mut s = PetStats { hunger: 0.0, happiness: 0.5, experience: 50.0, level: 1, bond: 0.5 };
        assert!(!s.level_up());
        assert_eq!(s.level, 1);
    }

    #[test]
    fn stats_clamped() {
        let mut s = PetStats { hunger: 1.0, happiness: 0.0, experience: 0.0, level: 1, bond: 0.0 };
        s.play();
        s.play();
        s.play();
        s.play();
        s.play();
        assert!(s.happiness <= 1.0);
        let mut s2 = PetStats { hunger: 0.0, happiness: 1.0, experience: 0.0, level: 1, bond: 1.0 };
        s2.feed();
        assert!(s2.hunger >= 0.0);
    }

    // -- PetMood --
    #[test]
    fn mood_hungry() {
        let s = PetStats { hunger: 0.9, happiness: 0.5, experience: 0.0, level: 1, bond: 0.5 };
        assert_eq!(PetMood::from_stats(&s), PetMood::Hungry);
    }

    #[test]
    fn mood_loyal() {
        let s = PetStats { hunger: 0.2, happiness: 0.9, experience: 0.0, level: 1, bond: 0.8 };
        assert_eq!(PetMood::from_stats(&s), PetMood::Loyal);
    }

    #[test]
    fn mood_happy() {
        let s = PetStats { hunger: 0.2, happiness: 0.8, experience: 0.0, level: 1, bond: 0.3 };
        assert_eq!(PetMood::from_stats(&s), PetMood::Happy);
    }

    #[test]
    fn mood_excited() {
        let s = PetStats { hunger: 0.2, happiness: 0.5, experience: 60.0, level: 1, bond: 0.3 };
        assert_eq!(PetMood::from_stats(&s), PetMood::Excited);
    }

    #[test]
    fn mood_curious() {
        let s = PetStats { hunger: 0.2, happiness: 0.4, experience: 0.0, level: 1, bond: 0.6 };
        assert_eq!(PetMood::from_stats(&s), PetMood::Curious);
    }

    #[test]
    fn mood_sleepy() {
        let s = PetStats { hunger: 0.2, happiness: 0.3, experience: 0.0, level: 1, bond: 0.2 };
        assert_eq!(PetMood::from_stats(&s), PetMood::Sleepy);
    }

    // -- Pet --
    #[test]
    fn pet_new() {
        let p = Pet::new("Fluffy", PetSpecies::Cat);
        assert_eq!(p.name, "Fluffy");
        assert_eq!(p.species, PetSpecies::Cat);
        assert!(p.tricks.is_empty());
        assert!(p.accessories.is_empty());
    }

    #[test]
    fn teach_trick() {
        let mut p = Pet::new("Fluffy", PetSpecies::Cat);
        p.teach_trick("sit");
        assert_eq!(p.tricks, vec!["sit"]);
        p.teach_trick("sit"); // no duplicates
        assert_eq!(p.tricks, vec!["sit"]);
        assert_eq!(p.stats.experience, 10.0); // only one train
    }

    #[test]
    fn add_accessory() {
        let mut p = Pet::new("Fluffy", PetSpecies::Cat);
        p.add_accessory("bow");
        assert_eq!(p.accessories, vec!["bow"]);
        p.add_accessory("bow"); // no duplicates
        assert_eq!(p.accessories, vec!["bow"]);
    }

    #[test]
    fn appearance_includes_details() {
        let mut p = Pet::new("Fluffy", PetSpecies::Cat);
        p.add_accessory("bow");
        p.teach_trick("sit");
        let desc = p.appearance();
        assert!(desc.contains("Fluffy"));
        assert!(desc.contains("Cat"));
        assert!(desc.contains("bow"));
        assert!(desc.contains("sit"));
    }

    #[test]
    fn pet_serde_roundtrip() {
        let p = Pet::new("Fluffy", PetSpecies::Dragon);
        let json = serde_json::to_string(&p).unwrap();
        let back: Pet = serde_json::from_str(&json).unwrap();
        assert_eq!(p, back);
    }

    // -- PetEvolution --
    #[test]
    fn evolution_can_evolve_check() {
        let evo = PetEvolution {
            from_species: PetSpecies::Cat,
            to_species: PetSpecies::Unicorn,
            min_level: 5,
            min_bond: 0.5,
        };
        let mut p = Pet::new("Fluffy", PetSpecies::Cat);
        assert!(!evo.can_evolve(&p)); // level too low

        p.stats.level = 5;
        p.stats.bond = 0.5;
        assert!(evo.can_evolve(&p));
    }

    #[test]
    fn evolution_perform() {
        let evo = PetEvolution {
            from_species: PetSpecies::Cat,
            to_species: PetSpecies::Unicorn,
            min_level: 5,
            min_bond: 0.5,
        };
        let mut p = Pet::new("Fluffy", PetSpecies::Cat);
        p.stats.level = 5;
        p.stats.bond = 0.6;
        assert!(evo.evolve(&mut p));
        assert_eq!(p.species, PetSpecies::Unicorn);
        assert_eq!(p.stats.happiness, 1.0);
    }

    #[test]
    fn evolution_fails_wrong_species() {
        let evo = PetEvolution {
            from_species: PetSpecies::Cat,
            to_species: PetSpecies::Unicorn,
            min_level: 5,
            min_bond: 0.5,
        };
        let mut p = Pet::new("Fluffy", PetSpecies::Wolf);
        p.stats.level = 10;
        p.stats.bond = 0.9;
        assert!(!evo.evolve(&mut p));
    }

    #[test]
    fn all_evolutions_count() {
        assert_eq!(PetEvolution::all_evolutions().len(), 5);
    }

    // -- PetManager --
    #[test]
    fn manager_adopt_and_get() {
        let mut m = PetManager::new();
        m.adopt("alice", "Fluffy", PetSpecies::Cat);
        let p = m.get_pet("alice").unwrap();
        assert_eq!(p.name, "Fluffy");
        assert!(m.get_pet("bob").is_none());
    }

    #[test]
    fn manager_adopt_idempotent() {
        let mut m = PetManager::new();
        m.adopt("alice", "Fluffy", PetSpecies::Cat);
        m.adopt("alice", "Spot", PetSpecies::Wolf); // should not overwrite
        let p = m.get_pet("alice").unwrap();
        assert_eq!(p.name, "Fluffy");
    }

    #[test]
    fn manager_evolve_pet() {
        let mut m = PetManager::new();
        m.adopt("alice", "Fluffy", PetSpecies::Cat);
        // Level up to 5
        {
            let pet = m.get_pet_mut("alice").unwrap();
            pet.stats.level = 5;
            pet.stats.bond = 0.6;
        }
        assert!(m.can_evolve_pet("alice"));
        assert!(m.evolve_pet("alice"));
        assert_eq!(m.get_pet("alice").unwrap().species, PetSpecies::Unicorn);
    }

    #[test]
    fn manager_cannot_evolve_no_pet() {
        let m = PetManager::new();
        assert!(!m.can_evolve_pet("nobody"));
    }

    #[test]
    fn manager_evolve_pet_fails_low_level() {
        let mut m = PetManager::new();
        m.adopt("alice", "Fluffy", PetSpecies::Cat);
        assert!(!m.can_evolve_pet("alice"));
        assert!(!m.evolve_pet("alice"));
    }

    #[test]
    fn manager_players() {
        let mut m = PetManager::new();
        m.adopt("alice", "Fluffy", PetSpecies::Cat);
        m.adopt("bob", "Spot", PetSpecies::Wolf);
        let mut players = m.players();
        players.sort();
        assert_eq!(players, vec!["alice", "bob"]);
    }
}
