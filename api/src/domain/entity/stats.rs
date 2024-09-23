pub struct Stats {
    hp: u8,
    attack: u8,
    defense: u8,
    sp_attack: u8,
    sp_defense: u8,
    speed: u8,
}

impl Stats {
    pub fn new(hp: u8, attack: u8, defense: u8, sp_attack: u8, sp_defense: u8, speed: u8) -> Self {
        Self {
            hp,
            attack,
            defense,
            sp_attack,
            sp_defense,
            speed,
        }
    }

    pub fn hp(&self) -> &u8 {
        &self.hp
    }

    pub fn attack(&self) -> &u8 {
        &self.attack
    }

    pub fn defense(&self) -> &u8 {
        &self.defense
    }

    pub fn sp_attack(&self) -> &u8 {
        &self.sp_attack
    }

    pub fn sp_defense(&self) -> &u8 {
        &self.sp_defense
    }

    pub fn speed(&self) -> &u8 {
        &self.speed
    }
}
