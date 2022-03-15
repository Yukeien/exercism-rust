// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn revive(&self) -> Option<Player> {
        let mut player = Some(self);

        if self.health > 0 {
            player = None
        }

        match player {
            Some(ply) => {
                let mut new_player = Player { health: 100, mana: None, level: ply.level };

                if ply.level >= 10 {
                    new_player.mana = Some(100);
                }

                Some(new_player)
            },
            None => None
        }
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        return match self.mana {
            Some(mana) => {
                if mana_cost > mana {
                    return 0;
                }

                self.mana = Some(mana - mana_cost);

                mana_cost * 2
            },
            None => {
                if self.health > mana_cost {
                    self.health -= mana_cost;
                } else {
                    self.health = 0;
                }

                0
            }
        }
    }
}
