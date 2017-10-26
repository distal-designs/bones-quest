pub enum BackgroundCommand {
    Hide,
    Show(String),
}

pub struct Command {
    background: Option<BackgroundCommand>,
    portrait: String,
    label: String,
    text: String,
    menu: String,
}
