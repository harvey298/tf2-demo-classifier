use std::{fs, io, path::Path};

use serde::{Serialize, Deserialize};
use tf_demo_parser::{Demo, DemoParser, demo::parser::gamestateanalyser::GameStateAnalyser};
use anyhow::Result;

fn main() {
    
    loop {
        println!("Path to Demo: ");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input = input.replace("\n", "").replace("\r", "");
        
        if Path::new(&input).exists() {
            open_demo(&input).unwrap();


        } else { println!("Invalid Path"); }

    }

}


fn open_demo(path: &str) -> Result<()> {

    let file = fs::read(path).unwrap();

    let demo = Demo::new(&file);
    let parser  = DemoParser::new_all_with_analyser(demo.get_stream(), GameStateAnalyser::default());
    let (header, mut state) = parser.ticker().unwrap();

    let server_name = header.server;
    let map = header.map;

    println!("Server Name: {server_name}");
    println!("Map: {map}\n");
    
    let mut cheaters = Vec::new();

    loop {
        match state.tick() {
            Ok(true) => {

                let state2 = state.state();                
                
                for (_, player) in state2.players.clone().into_iter().enumerate() {
                    if let Some(info) = &player.info {
                        let steam_id= info.steam_id.clone();
                        if steam_id == "BOT" { continue; }
                        let name = info.name.clone();
                        println!("Steam ID: {steam_id}, Name: {name}");

                        let mut input = String::new();
                        io::stdin().read_line(&mut input).unwrap();
                        input = input.replace("\n", "").to_lowercase();

                        if input.contains("y") { cheaters.push(steam_id); }

                        if input.contains("s") { break; }
                    }
                }

                
                break;
            }
            Ok(false) => {
                break;
            }
            Err(e) => {
                println!("Error: {e:?}");
                break;
            }
        }
    }

    let data = Label{ demo_file: path.to_owned(), cheater: !cheaters.is_empty(), cheater_steam_id: cheaters };

    let path = format!("{}_label.toml", path);
    let data = toml::to_string_pretty(&data).unwrap();
    fs::write(path, data).unwrap();    

    Ok(())
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Label {
    pub demo_file: String,
    pub cheater: bool,
    pub cheater_steam_id: Vec<String>,
}