use colored::Colorize;
use std::fs;
use std::path::Path;

#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
const RUNTIME_BINARY: &[u8] = include_bytes!("../runtimes/modu-runtime-linux-x64");

pub fn build() {
    #[cfg(not(all(target_os = "linux", target_arch = "x86_64")))]
    {
        println!("{}", "Building the modu files is only supported on Linux x86_64 for now.".red());
        return;
    }

    let args = std::env::args().collect::<Vec<String>>();

    let source_path = if args.len() < 3 || args[2].starts_with("--")  {
        "main.modu"
    } else {
        &args[2]
    };

    let output_name = args.iter()
        .position(|arg| arg == "--output" || arg == "-o")
        .and_then(|i| args.get(i + 1))
        .map(|s| s.as_str())
        .unwrap_or(source_path.split('/').last().unwrap_or("output").split('.').next().unwrap_or("output"));
    
    if !Path::new(source_path).exists() {
        println!("{}", format!("Source file not found: {}", source_path).red());
        return;
    }

    let mut output_binary = RUNTIME_BINARY.to_vec();

    output_binary.extend_from_slice(b"__BEGIN_MODU_EMBEDDED_CODE__");
    output_binary.extend_from_slice(fs::read(source_path).unwrap_or_else(|e| {
        println!("{}", format!("Failed to read source file: {}", e).red());
        std::process::exit(1);
    }).as_slice());
    output_binary.extend_from_slice(b"__END_MODU_EMBEDDED_CODE__");

    let ext = if cfg!(target_os = "windows") { ".exe" } else { "" };
    let output = format!("{}{}", output_name, ext);

    fs::write(&output, output_binary).unwrap_or_else(|e| {
        println!("{}", format!("Failed to write output file: {}", e).red());
        std::process::exit(1);
    });

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&output).unwrap().permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&output, perms).unwrap_or_else(|e| {
            println!("{}", format!("Failed to set permissions on output file: {}", e).red());
            std::process::exit(1);
        });
    }

    let size_mb = fs::metadata(&output).unwrap().len() as f64 / 1024.0 / 1024.0;
    println!("{}", format!("Successfully built '{}' ({:.2} MB)", output, size_mb).green());
}