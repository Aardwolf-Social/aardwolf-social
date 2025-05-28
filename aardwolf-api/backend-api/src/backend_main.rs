// aardwolf-api/backend-api/src/backend_main.rs
use aardwolf_api_common::config::load_config;

fn main() {
    let settings = load_config().expect("Failed to load config");

    println!("Backend engine: {}", settings.backend.engine);

    match settings.backend.engine.as_str() {
        "actix" => {
            if let Some(actix) = settings.actix {
                println!("Starting Actix on {}:{}", actix.host, actix.port);
            }
        }
        "warp" => {
            if let Some(warp) = settings.warp {
                println!("Starting Warp on {}:{}", warp.host, warp.port);
            }
        }
        _ => {
            println!("Unknown backend selected!");
        }
    }

    println!("Database engine: {}", settings.database.engine);
    println!("Frontend engine: {}", settings.frontend.engine);
}
