mod external; // the name of the "external" module must be listed in the root file - main or lib
mod side_module; // another module from the same dir can be referenced by its name

use external::modception; // "use" can set up a shortcut to a module within another module

mod internal_mod {
    pub fn say_hello() {
        println!("Hello!");
    }

    fn private_hello() {
        println!("I'm only accessible on the inside of the mod");
    }
}

fn main() {
    internal_mod::say_hello();
    external::hello_external();
    side_module::hello_side();
    modception::so_many_mods();
}
