pub fn init() {
    use std::io::Write;

    let mut package_name = String::new();
    let mut package_version = String::new();

    let mut is_library = String::new();

    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .append(false)
        .open("project.toml")
        .unwrap();

    if file.metadata().unwrap().len() > 0 {
        println!("Project already initialized");
        print!("Overwrite? (y/N) ");
        std::io::stdout().flush().unwrap();

        let mut overwrite = String::new();

        std::io::stdin().read_line(&mut overwrite).unwrap();

        if overwrite.trim() == "y" {
            file.set_len(0).unwrap();
        } else {
            println!("Aborted");
            return;
        }
    }
    
    print!("Enter package name: ");
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut package_name).unwrap();

    package_name = package_name.trim().to_string();

    if package_name.is_empty() {
        println!("Package name cannot be empty");
        return;
    }

    if !package_name.chars().all(|c| c.is_alphanumeric() || c == '_') {
        println!("Package name can only contain alphanumeric characters and underscores");
        return;
    }    

    print!("Enter package version: ");
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut package_version).unwrap();

    if package_version.trim().is_empty() {
        println!("Package version cannot be empty");
        return;
    }

    print!("Enter description (optional): ");
    std::io::stdout().flush().unwrap();
    let mut package_description = String::new();
    std::io::stdin().read_line(&mut package_description).unwrap();

    print!("Is this a library? (y/N) ");
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut is_library).unwrap();

    let package_name = package_name.trim();
    let package_version = package_version.trim();
    let package_description = package_description.trim();

    file.write_all(format!("[package]\nname = \"{}\"\nversion = \"{}\"\ndescription = \"{}\"\n", package_name, package_version, package_description).as_bytes()).unwrap();

    let mut file_name = "main.modu";

    if is_library.trim() == "y" {
        file_name = "lib.modu";
    }

    let mut main_file = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .append(false)
        .open(file_name)
        .unwrap();

    if is_library.trim() == "y" {
        main_file.write_all(b"fn hello() {\n    print(\"Hello, world!\");\n}").unwrap();
    } else {
        main_file.write_all("print(\"Hello, world!\");".as_bytes()).unwrap();
    }
}