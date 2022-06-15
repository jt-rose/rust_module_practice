// this file must be named "mod" to make it accessible as an entry point
// under this directory called external
// similar to dir/index.js in JavaScript

pub fn hello_external() {
    println!("Hi");
}

pub mod modception {
    pub fn so_many_mods() {
        println!("modtacular!");
    }
}