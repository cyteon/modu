pub fn help() {
    let args = std::env::args().collect::<Vec<String>>();

    let stdlib = if args.len() > 2 {
        &args[2]
    } else {
        println!("Usage: modu help <stdlib>");
        println!("Standard library modules:");
        println!("  crypto");
        println!("  encoding");
        println!("  ffi");
        println!("  fs");
        println!("  http");
        println!("  json");
        println!("  math");
        println!("  os");
        println!("  time");
        println!("  uuid");
        return;
    };

    let contents = match stdlib.as_str() {
        "crypto"   => include_str!("../stdlib/crypto.txt"),
        "encoding" => include_str!("../stdlib/encoding.txt"),
        "ffi"      => include_str!("../stdlib/ffi.txt"),
        "fs"       => include_str!("../stdlib/fs.txt"),
        "http"     => include_str!("../stdlib/http.txt"),
        "json"     => include_str!("../stdlib/json.txt"),
        "math"     => include_str!("../stdlib/math.txt"),
        "os"       => include_str!("../stdlib/os.txt"),
        "time"     => include_str!("../stdlib/time.txt"),
        "uuid"     => include_str!("../stdlib/uuid.txt"),
        stdlib => {
            println!("Unknown standard library module: {}", stdlib);
            return;
        }
    };

    println!("{}", contents);
}