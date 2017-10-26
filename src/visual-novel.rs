pub enum BackgroundCommand {
    Hide,
    Show(String),
}

pub struct Command {
    background: String,
    portrait: String,
    label: String,
    text: String,
    menu: String,
}
