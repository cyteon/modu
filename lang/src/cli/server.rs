use rouille::router;
use crate::utils;
use crate::parser::parse;
use std::sync::Arc;

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

                let context = &mut utils::create_context();

                std::io::set_output_capture(Some(Default::default()));

                parse(&text, context).unwrap_or_else(|e| {
                    println!("\n⚠️ {}", e.0);
                    println!("Traceback (most recent call last):");
                    println!("    File \"<stdin>\", line {}", e.1);
                    println!("Believe this is a bug? Report it: https://github.com/cyteon/modu/issues/new");
                });

                let captured = String::from_utf8(
                    Arc::try_unwrap(
                        std::io::set_output_capture(None).unwrap()
                    )
                        .unwrap()
                        .into_inner()
                        .unwrap()
                ).unwrap();

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