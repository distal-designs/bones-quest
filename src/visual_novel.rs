extern crate toml;



#[derive(Deserialize, Debug)]
pub enum BackgroundCommand {
    Hide,
    Show(String),
}


#[derive(Deserialize, Debug)]
pub enum PortraitCommand {
    Hide,
    Show(String, String)
}


#[derive(Deserialize, Debug)]
pub struct Command {
    background: Option<BackgroundCommand>,
    portrait: Option<PortraitCommand>,
    text: String,
}
