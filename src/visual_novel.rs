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



impl Command {
    pub fn parse (toml: &str) -> Result<Vec<Command>, toml::de::Error>{
        #[derive(Deserialize)]
        pub struct Commands {
            command: Vec<Command>,
        }

        toml::from_str::<Commands>(toml).map(|commands| commands.command)
    }
}
