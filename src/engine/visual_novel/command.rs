extern crate toml;


use std::collections::HashMap;
use std::io::Read;

use ggez::filesystem::Filesystem;


#[derive(Deserialize, Debug)]
#[serde(tag = "t", content = "c")]
pub enum Background {
    Hide,
    Color(String),
    Image(String),
}


#[derive(Deserialize, Debug)]
#[serde(tag = "t", content = "c")]
pub enum Portrait {
    Hide,
    Show(String, String),
}


#[derive(Deserialize, Debug)]
pub struct Position {
    pub direction: String,
    pub position: i8,
}


#[derive(Deserialize, Debug)]
pub struct Command {
    pub background: Option<Background>,
    pub portrait: Option<Portrait>,
    pub text: String,
    pub id: Option<String>,
    pub positions: Option<HashMap<String, Position>>,
    pub menu: Option<HashMap<String, String>>,
}


impl Command {
    fn parse(toml: &str) -> Result<Vec<Self>, toml::de::Error> {
        #[derive(Deserialize)]
        struct Commands {
            command: Vec<Command>,
        }

        let t = toml::from_str::<Commands>(toml).map(|commands| commands.command);
        println!("{:#?}", t);
        t
    }


    pub fn load(fs: &mut Filesystem, path: &str) -> Result<Vec<Self>, toml::de::Error> {
        let mut f = fs.open(path).unwrap();
        let mut toml = String::new();
        f.read_to_string(&mut toml).unwrap();

        Self::parse(&toml)
    }
}
