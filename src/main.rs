#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
mod routes;


fn main() {
    let server = start_rocket_server();
    server.launch();
}

fn start_rocket_server() -> rocket::Rocket {
    return rocket::ignite().mount("/", routes![routes::index]);
}


