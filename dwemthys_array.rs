// idea by _why (http://mislav.uniqpath.com/poignant-guide/dwemthy/)

struct Monster {
    health: int,
    attack: int
}

impl Monster {
    // associated functions are like php static functions
    // associated functions make good constructors
    fn new(health: int, attack: int) -> Monster {
        // Note the lack of a semicolon inside new,
        // so it’s acting as an expression.
        // new is just a function that creates a new
        // Monster struct and returns it.
        Monster { health:health, attack:attack }
    }

    // methods take a borrowed pointer to self
    fn attack(&self) {
        println!("The monster attacks for {:d} damage!", self.attack);
    }

    fn count() {
        println!("There are a bunch of monsters out tonight.");
    }
}

fn main() {
    let m = Monster { health: 10, attack: 20 };

    // invoke method
    m.attack();

    // invoke associated function
    Monster::count();

    // construct a monster
    Monster::new(20, 40).attack();
}
