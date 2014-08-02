struct Monster {
    health: int,
    attack: int
}

fn main() {
    let m = Monster { health: 10, attack: 20 };

    println!("{:s}", m.health.to_str());
    println!("{:s}", m.attack.to_str());
}
