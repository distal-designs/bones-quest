extern crate toml;

use std::collections::HashMap;
use std::io::Read;

use ggez::filesystem::Filesystem;

#[derive(Deserialize, Debug)]
#[serde(tag = "t", content = "c")]
pub enum BackgroundCommand {
    Hide,
    Color(String),
    Image(String),
}

#[derive(Deserialize, Debug)]
#[serde(tag = "t", content = "c")]
pub enum PortraitCommand {
    Hide,
    Show(String, String),
}

#[derive(Deserialize, Debug)]
pub struct PositionCommand {
    pub direction: String,
    pub position: i8,
}

#[derive(Deserialize, Debug)]
pub struct Command {
    pub background: Option<BackgroundCommand>,
    pub portrait: Option<PortraitCommand>,
    pub text: String,
    pub positions: Option<HashMap<String, PositionCommand>>,
}

impl Command {
    fn parse(toml: &str) -> Result<Vec<Self>, toml::de::Error> {
        #[derive(Deserialize)]
        struct Commands {
            command: Vec<Command>,
        }

        toml::from_str::<Commands>(toml).map(|commands| commands.command)
    }

    pub fn load(fs: &mut Filesystem, path: &str) -> Result<Vec<Self>, toml::de::Error> {
        let mut f = fs.open(path).unwrap();
        let mut toml = String::new();
        f.read_to_string(&mut toml).unwrap();

        Self::parse(&toml)
    }
}
