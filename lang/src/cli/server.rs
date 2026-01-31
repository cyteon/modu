use gag::BufferRedirect;
use rouille::router;
use std::io::Read;
use crate::parser::parse;

pub fn server() {
    let args = std::env::args().collect::<Vec<String>>();

    let mut port = 2424;

    if args.len() > 2 {
        port = args[2].parse::<u16>().unwrap();
    }

    println!("Modu server starting on port {}", port);

    rouille::start_server(format!("0.0.0.0:{}", port), move |request| {
        router!(request,
            (GET) (/) => {
                rouille::Response {
                    status_code: 200,
                    headers: vec![
                        ("Content-Type".into(), "text/html".into()),
                        ("Access-Control-Allow-Origin".into(), "*".into()),
                        ("Access-Control-Allow-Methods".into(), "GET, POST, OPTIONS".into()),
                        ("Access-Control-Allow-Headers".into(), "Content-Type".into()),
                    ],
                    data: rouille::ResponseBody::from_string(format!("OK v{}", env!("CARGO_PKG_VERSION"))),
                    upgrade: None
                }
            },

            (POST) (/eval) => {
                println!("POST /eval | {} | {}", request.remote_addr(), request.header("User-Agent").unwrap_or("unknown"));

                let text = rouille::input::plain_text_body(request).unwrap();

                if text.contains("exit") {
                    return rouille::Response {
                        status_code: 200,
                        headers: vec![
                            ("Content-Type".into(), "text/plain".into()),
                            ("Access-Control-Allow-Origin".into(), "*".into()),
                            ("Access-Control-Allow-Methods".into(), "GET, POST, OPTIONS".into()),
                            ("Access-Control-Allow-Headers".into(), "Content-Type".into()),
                        ],
                        data: rouille::ResponseBody::from_string("exit() is disabled on the server".to_string()),
                        upgrade: None
                    };
                }

                let context = &mut crate::utils::create_context();

                let mut stdout = BufferRedirect::stdout().unwrap();
                let mut stderr = BufferRedirect::stderr().unwrap();

                parse(&text, "<server>", context);

                let mut out = String::new();
                let mut err = String::new();

                stdout.read_to_string(&mut out).unwrap();
                stderr.read_to_string(&mut err).unwrap();

                let captured = format!("{}{}", out, err);

                rouille::Response {
                    status_code: 200,
                    headers: vec![
                        ("Content-Type".into(), "text/plain".into()),
                        ("Access-Control-Allow-Origin".into(), "*".into()),
                        ("Access-Control-Allow-Methods".into(), "GET, POST, OPTIONS".into()),
                        ("Access-Control-Allow-Headers".into(), "Content-Type".into()),
                    ],
                    data: rouille::ResponseBody::from_string(captured),
                    upgrade: None
                }
            },

            (OPTIONS) (/eval) => {
                rouille::Response {
                    status_code: 200,
                    headers: vec![
                        ("Access-Control-Allow-Origin".into(), "*".into()),
                        ("Access-Control-Allow-Methods".into(), "GET, POST, OPTIONS".into()),
                        ("Access-Control-Allow-Headers".into(), "Content-Type".into()),
                    ],
                    data: rouille::ResponseBody::empty(),
                    upgrade: None
                }
            },

            _ => {
                rouille::Response::empty_404()
            }
        )
    });
}