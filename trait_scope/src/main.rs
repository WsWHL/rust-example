mod fruit;
mod animal;

use std::ops::Add;
use crate::fruit::{{ Food, Apple }};
use crate::animal::{{Bob}, Programmer};

struct Game;
struct Enemy;
struct Hero;

// Loadable特征
trait Loadable {
    fn init(&self);
}

impl Loadable for Enemy {
    fn init(&self) {
        println!("Enemy loaded");
    }
}

impl Loadable for Hero {
    fn init(&self) {
        println!("Hero loaded");
    }
}

impl Game {
    fn load<T: Loadable>(&self, entity: T) {
        entity.init();
        println!("Game load completed!");
    }
}

fn add_thing<T: Add>(fst: T, snd: T) {
    let _ = fst + snd;
}

fn main() {
    println!("范型函数和impl特征区间:");

    let game = Game;
    game.load(Enemy);
    game.load(Hero);

    add_thing(2, 9);

    let foot = Food(Apple);
    fruit::eat(foot);

    println!("组合特征区间:");
    Bob.animate();
}
