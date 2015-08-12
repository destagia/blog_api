#[macro_use] extern crate nickel;
extern crate rustc_serialize;
extern crate postgres;
extern crate time;

mod db;
mod model;
mod timeutil;

use rustc_serialize::json;
use model::Postgresable;
use nickel::{Nickel, HttpRouter};

fn main() {
    let mut server = Nickel::new();

    server.get("/accounts", middleware! { |req|
        let mut v: Vec<model::Account> = Vec::new();
        &db::select(&mut v);
        json::encode(&v).unwrap()
    });

    server.listen("127.0.0.1:6767");
}