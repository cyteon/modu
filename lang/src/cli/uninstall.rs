pub fn uninstall() {
    let args = std::env::args().collect::<Vec<String>>();

    if args.len() < 3 {
        println!("Usage: modu uninstall <name>");
        return;
    }

    let name = &(args[2].clone().split("@").collect::<Vec<&str>>()[0].to_string());

    if !std::fs::exists(".modu/packages/".to_string() + name).unwrap() {
        println!("Package {} is not installed", name);
        return;
    }

    std::fs::remove_dir_all(".modu/packages/".to_string() + name).unwrap();

    println!("Package {} uninstalled", name);
}
