use crate::{grid::Grid, protocol::Hero};

pub struct Hero_s
{
    pub id: i32,
    pub owner_id: i32,
    pub type_: String,
    pub x: i32,
    pub y: i32,
    pub hp: i32,
    pub cooldown: i32,
    pub state: String, // "patrol down", "patrol up", "enemy spotted"
    pub hero: Hero,
}

impl Hero_s
{
    pub fn new(hero: &Hero) -> Self
    {
        Hero_s {
            id: hero.id,
            owner_id: hero.owner_id,
            type_: hero.type_.clone(),
            x: hero.x,
            y: hero.y,
            hp: hero.hp,
            cooldown: hero.cooldown,
            state: "patrol down".to_string(),
            hero: hero.clone(),
        }
    }
    
    fn hero_go_right(&self) -> (i32, i32) {
        (self.x + 3, self.y)
    }
    fn hero_go_left(&self) -> (i32, i32) {
        (self.x - 3, self.y)
    }
    fn hero_go_up(&self) -> (i32, i32) {
        (self.x, self.y - 3)
    }
    fn hero_go_down(&self) -> (i32, i32) {
        (self.x, self.y + 3)
    }
    fn hero_go_up_right(&self) -> (i32, i32) {
        (self.x + 3, self.y - 3)
    }
    fn hero_go_up_left(&self) -> (i32, i32) {
        (self.x - 3, self.y - 3)
    }
    fn hero_go_down_right(&self) -> (i32, i32) {
        (self.x + 3, self.y + 3)
    }
    fn hero_go_down_left(&self) -> (i32, i32) {
        (self.x - 3, self.y + 3)
    }

    pub fn hero_logic(&mut self, grid: &Grid) -> (i32, i32) {
        //Go to the enemy spawn, if you see an enemy hero, try to shoot it(if in line of sight), and run away
        //If on the last row, go up, if on the first row, go down
        //Order: shoot, run away, patrol the map

        if grid.has_enemy_hero(&self.hero)    
        {
            //Shoot the enemy hero if in line of sight
            //Run away in the opposite direction of the enemy hero
            return (self.x, self.y); // Placeholder, replace with actual logic
        } 
        else
        {
            if self.state == "patrol down" {
                if self.y >= 27 {
                    self.state = "patrol up".to_string();
                    return self.hero_go_up();
                } else {
                    return self.hero_go_down();
                }
            } else 
            {
                if self.y <= 3 {
                    self.state = "patrol down".to_string();
                    return self.hero_go_down();
                } else {
                    return self.hero_go_up();
                }
            }
        }
    }
}