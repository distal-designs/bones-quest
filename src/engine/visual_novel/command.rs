extern crate toml;

use std::env;
use std::fs::File;
use std::io::Read;

#[derive(Deserialize, Debug)]
#[serde(tag = "t", content = "c")]
pub enum BackgroundCommand {
    Hide,
    Color(String),
    Image(String),
}

#[derive(Deserialize, Debug)]
pub enum PortraitCommand {
    Hide,
    Show(String, String),
}

#[derive(Deserialize, Debug)]
pub struct Command {
    background: Option<BackgroundCommand>,
    portrait: Option<PortraitCommand>,
    pub text: String,
}

impl Command {
    fn parse(toml: &str) -> Result<Vec<Command>, toml::de::Error> {
        #[derive(Deserialize)]
        struct Commands {
            command: Vec<Command>,
        }

        toml::from_str::<Commands>(toml).map(|commands| commands.command)
    }

    pub fn load(dialog_name: &str) -> Result<Vec<Command>, toml::de::Error> {
        let mut pathbuf = env::current_dir().unwrap();
        pathbuf.push("resources");
        pathbuf.push("dialogs");
        pathbuf.push(format!("{}.toml", dialog_name));
        let path = pathbuf.as_path();
        let mut f = File::open(path).unwrap();
        let mut toml = String::new();
        f.read_to_string(&mut toml).unwrap();

        Command::parse(&toml)
    }
}
