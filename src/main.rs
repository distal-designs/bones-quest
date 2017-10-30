#[macro_use]
extern crate serde_derive;

mod visual_novel;

fn main() {
    let dialog = visual_novel::Command::parse(r#"
    [[command]]
    text = "this is some text"

    [[command]]
    text = "so is this"
    "#);
    println!("{:?}", dialog);
}
