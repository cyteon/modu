use reqwest;
use std::{env, io::Read, io::Write};

pub fn login() {
    let path;

    if cfg!(windows) {
        let home = env::var("USERPROFILE").unwrap();

        path = format!("{}\\.modu\\config.toml", home);

        std::fs::create_dir_all(format!("{}\\.modu", home)).unwrap();
    } else {
        let home = env::var("HOME").unwrap();

        path = format!("{}/.modu/config.toml", home);

        std::fs::create_dir_all(format!("{}/.modu", home)).unwrap();
    }

    let mut config_file = std::fs::OpenOptions::new()
        .write(true)
        .read(true)
        .create(true)
        .open(path.clone())
        .unwrap();

    let mut config_file_contents = String::new();
    config_file
        .read_to_string(&mut config_file_contents)
        .unwrap();

    if config_file_contents.len() > 0 {
        use std::io::Write;

        println!("Already logged in");
        print!("Overwrite? (y/N) ");
        std::io::stdout().flush().unwrap();

        let mut overwrite = String::new();
        std::io::stdin().read_line(&mut overwrite).unwrap();

        if overwrite.trim() != "y" {
            println!("Aborted");
            return;
        }
    }

    let mut config_file = std::fs::OpenOptions::new()
        .write(true)
        .read(true)
        .create(true)
        .truncate(true)
        .open(path)
        .unwrap();

    let toml = toml::from_str::<toml::Value>(&"").unwrap();
    let mut toml = toml.as_table().unwrap().clone();

    let mut use_diffrent_backend = String::new();
    print!("Use different backend? (y/N) ");
    std::io::stdout().flush().unwrap();

    std::io::stdin()
        .read_line(&mut use_diffrent_backend)
        .unwrap();

    let mut backend_url = String::new();

    if use_diffrent_backend.trim() == "y" {
        print!("Enter backend URL: ");
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut backend_url).unwrap();

        backend_url = backend_url.trim().trim_end_matches("/").to_string();

        toml.insert(
            "backend".to_string(),
            toml::Value::String(backend_url.clone()),
        );
    } else {
        backend_url = "https://mpm.cyteon.dev".to_string();
        toml.insert(
            "backend".to_string(),
            toml::Value::String(backend_url.clone()),
        );
    }

    println!("Paste the code from {}/token", backend_url);

    let mut token = String::new();
    std::io::stdin().read_line(&mut token).unwrap();

    let token = token.trim();

    let client = reqwest::blocking::Client::new();
    let res = client
        .get(&format!("{}/api/v1/code/verify", backend_url))
        .header("Authorization", token)
        .send()
        .unwrap();

    if res.status().as_u16() != 200 {
        println!("{}", res.text().unwrap());
        return;
    }

    let user_id = res.text().unwrap();

    println!("Authenticated as user with ID {}", user_id);

    toml.insert("token".to_string(), toml::Value::String(token.to_string()));

    let toml = toml::to_string(&toml).unwrap();

    config_file.write_all(toml.as_bytes()).unwrap();
}
