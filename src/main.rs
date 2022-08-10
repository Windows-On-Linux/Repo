use std::io::{stdin, Write};
use std::fs;
use sargparse::ArgumentParser;
use serde_json::{Value};
use serde_json::json;

fn main() {

    let mut parser = ArgumentParser::new(Some("WRU - Wineget Repo Updater"));

    let contents = fs::read_to_string("./repository.json")
        .expect("Something went wrong reading the file");

    let mut jsonstring: Value = serde_json::from_str(&contents).unwrap();
    let mut jsonarray = jsonstring.as_array_mut().unwrap();
    let mut appname = String::new();
    println!("Please enter the name of the app that you want to add");
    stdin().read_line(&mut appname)
    	.ok()
        .expect("Failed to read line");
    let mut cliname = String::new();
    println!("Please enter the name that your app will have in Wineget CLI");
    stdin().read_line(&mut cliname)
        .ok()
        .expect("Failed to read line");
    println!("Now enter the description of your application");
    let mut description = String::new();
    stdin().read_line(&mut description)
        .ok()
        .expect("Failed to read line");
    let mut repository = String::new();
    println!("Enter the url of GIT repository where your script live");
    stdin().read_line(&mut repository)
        .ok()
        .expect("Failed to read line");
    let mut path = String::new();
    println!("Please enter the path where your script install the Wine application");
    stdin().read_line(&mut path)
        .ok()
        .expect("Failed to read line");
    let mut icon = String::new();
    println!("Please enter the URL of icon of Application");
    stdin().read_line(&mut icon)
        .ok()
        .expect("Failed to read line");
    let mut author = String::new();
    println!("Write the author of script");
    stdin().read_line(&mut author)
        .ok()
        .expect("Failed to read line");
    let mut version = String::new();
    println!("Write the version of script");
    stdin().read_line(&mut version)
        .ok()
        .expect("Failed to read line");
    jsonarray.push(json!(
        {
            "App": appname.replace("\n", ""),
            "CliName": cliname.replace("\n", ""),
            "Description": description.replace("\n", ""),
            "Repository": repository.replace("\n", ""),
            "Path": path.replace("\n", ""),
            "Icon": icon.replace("\n", ""),
            "Author": author.replace("\n", ""),
            "Version": version.replace("\n", "")
        }
    ));
    
    let mut repofile = std::fs::OpenOptions::new().write(true).open("./repository.json").unwrap();
    repofile.write_all(serde_json::to_string_pretty(jsonarray).unwrap().as_bytes()).unwrap();
    repofile.flush().unwrap();
}
