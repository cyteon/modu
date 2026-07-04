use std::io::{Read, Write};

use toml;

fn install_package(backend: &str, name: &str, version: &str) -> Result<serde_json::Value, String> {
    let client = reqwest::blocking::Client::new();

    let response = client
        .get(&format!(
            "{}/api/v1/packages/{}/{}?isDownload=true",
            backend, name, version
        ))
        .send()
        .unwrap();

    if response.status().as_u16() != 200 {
        let text = response.text().unwrap();

        println!("Error: {}", text);

        return Err(text);
    }

    let package = response.json::<serde_json::Value>().unwrap();

    println!(
        "Installing package {}@{}",
        name,
        package["version"].as_str().unwrap()
    );
    println!(
        "> {}",
        match package["description"].as_str() {
            Some(description) => description,
            None => "No description",
        }
    );

    let zip = client
        .get(package["zipUrl"].as_str().unwrap())
        .send()
        .unwrap()
        .bytes()
        .unwrap();

    let mut archive = zip::ZipArchive::new(std::io::Cursor::new(zip)).unwrap();

    if std::fs::exists(".modu/packages/".to_string() + name).unwrap() {
        std::fs::remove_dir_all(".modu/packages/".to_string() + name).unwrap();
    }

    std::fs::create_dir_all(".modu/packages/".to_string() + name).unwrap();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        let path = ".modu/packages/".to_string() + name + "/" + file.name();

        if file.name().contains("/") || file.name().contains("\\") {
            if file.name().contains("/") {
                let mut path = path.split("/").collect::<Vec<&str>>();
                path.pop();

                std::fs::create_dir_all(path.join("/")).unwrap();
            } else {
                let mut path = path.split("\\").collect::<Vec<&str>>();
                path.pop();

                std::fs::create_dir_all(path.join("\\")).unwrap();
            }

            #[cfg(windows)]
            let path =
                (".modu/packages/".to_string() + name + "/" + file.name()).replace("/", "\\");

            #[cfg(not(windows))]
            let path =
                (".modu/packages/".to_string() + name + "/" + file.name()).replace("\\", "/");

            let mut out = std::fs::File::create(path).unwrap();
            std::io::copy(&mut file, &mut out).unwrap();
        } else {
            #[cfg(windows)]
            let path = path.replace("/", "\\");

            #[cfg(not(windows))]
            let path = path.replace("\\", "/");

            let mut out = std::fs::File::create(path).unwrap();
            std::io::copy(&mut file, &mut out).unwrap();
        }
    }

    Ok(package)
}

pub fn install() {
    let mut content = String::new();
    let file = std::fs::File::open("project.toml");

    if file.is_err() {
        println!("No project.toml found. Run `modu init` to create a new project");
        return;
    }

    file.unwrap().read_to_string(&mut content).unwrap();

    let args = std::env::args().collect::<Vec<String>>();

    let toml = toml::from_str::<toml::Value>(&content).unwrap();
    let mut toml = toml.as_table().unwrap().clone();

    let dependencies = match toml.get_mut("dependencies") {
        Some(dependencies) => dependencies.as_table_mut().unwrap(),

        None => {
            toml.insert(
                "dependencies".to_string(),
                toml::Value::Table(toml::value::Table::new()),
            );
            toml.get_mut("dependencies")
                .unwrap()
                .as_table_mut()
                .unwrap()
        }
    };

    let mut backend_url = "https://mpm.cyteon.dev".to_string();

    let config_file;
    let path;

    if cfg!(windows) {
        let home = std::env::var("USERPROFILE").unwrap();
        path = format!("{}\\.modu\\config.toml", home);
    } else {
        let home = std::env::var("HOME").unwrap();
        path = format!("{}/.modu/config.toml", home);
    }

    config_file = std::fs::OpenOptions::new()
        .write(true)
        .read(true)
        .create(true)
        .open(path.clone());

    if config_file.is_ok() {
        let mut config_file_content = String::new();
        config_file
            .unwrap()
            .read_to_string(&mut config_file_content)
            .unwrap();

        if config_file_content.len() > 0 {
            let config_toml = toml::from_str::<toml::Value>(&config_file_content).unwrap();
            let table = config_toml.as_table().unwrap();

            backend_url = match table.get("backend") {
                Some(backend) => backend.to_string().replace("\"", ""),
                None => backend_url,
            };
        }
    }

    println!("Installing packages, using backend {}\n", backend_url);

    if args.len() < 3 {
        for (name, version) in dependencies.iter() {
            match install_package(&backend_url, name, version.as_str().unwrap()) {
                Ok(_) => {
                    println!("Package {} installed\n", name);
                }
                Err(_) => {
                    println!("Failed to install package {}", name);
                }
            };
        }

        return;
    }

    let name = &(args[2].clone().split("@").collect::<Vec<&str>>()[0].to_string());

    let version = match args[2].clone().split("@").collect::<Vec<&str>>().len() {
        1 => "latest".to_string(),
        _ => args[2].clone().split("@").collect::<Vec<&str>>()[1].to_string(),
    };

    let package = match install_package(&backend_url, name, &version) {
        Ok(package) => package,
        Err(_) => {
            println!("Failed to install package {}", name);
            return;
        }
    };

    dependencies.insert(
        name.clone(),
        toml::Value::String(package["version"].as_str().unwrap().to_string()),
    );

    let mut file = std::fs::File::create("project.toml").unwrap();
    file.write_all(toml::to_string(&toml).unwrap().as_bytes())
        .unwrap();

    println!("Package {} installed", name);
}
