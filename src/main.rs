#![allow(non_snake_case)]
use std::{fs, io, path::Path};

use tf_demo_parser::{Demo, DemoParser, demo::parser::gamestateanalyser::GameStateAnalyser};
use anyhow::Result;

use crate::aitl::{Label, save_AiTL_file, AiTLFileHeader, extract_AiTL_file};
mod aitl;

fn main() {

    println!("Output Folder: ");
    let mut output = String::new();
    io::stdin().read_line(&mut output).unwrap();
    output = output.replace("\n", "").replace("\r", "").replace("\\", "/");
    
    loop {
        println!("Path to Demo or AiTL file: ");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input = input.replace("\n", "").replace("\r", "").replace("\\", "/");

        if input.to_lowercase().contains(".aitl") {
            extract_AiTL_file(&input, &output).unwrap();
            continue;            
        }
        
        if Path::new(&input).exists() {
            let filename = input.split("/").last().unwrap().replace(".dem", ".aitl");
            let output_file = format!("{output}./{filename}");

            open_demo(&input, &output_file).unwrap();

        } else { println!("Invalid Path"); }

    }

}


fn open_demo(path: &str, output: &str) -> Result<()> {

    let file = fs::read(path).unwrap();

    let filename = path.split("/").last().unwrap();

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

    let data = Label{ demo_file: filename.to_owned(), cheater: !cheaters.is_empty(), cheater_steam_id: cheaters.to_vec() };

    let filename = filename.replace(".dem", ".toml");

    let header = AiTLFileHeader {
        label: data,
        label_filename: filename.to_owned(),
    };

    save_AiTL_file(output, &file, header).unwrap();

    Ok(())
}

#[cfg(test)]
mod test {
    
    use crate::aitl::{ extract_AiTL_file};

    #[test]
    #[allow(non_snake_case)]
    fn AiTL_load() {

        extract_AiTL_file("data.AITL", "./").unwrap();
    }

}