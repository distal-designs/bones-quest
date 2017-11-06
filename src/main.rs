#[macro_use]
extern crate serde_derive;

mod visual_novel;


struct MainState {
}


fn main() {
    println!("{:?}", visual_novel::Command::load("blood"));
}
