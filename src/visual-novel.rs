pub enum BackgroundCommand {
    Hide,
    Show(String),
}

pub enum PortraitCommand {
    Hide,
    Show(String, String)
}

pub struct Command {
    background: Option<BackgroundCommand>,
    portrait: Option<PortraitCommand>,
    label: Option<String>,
    text: String,
    menu: String,
}
