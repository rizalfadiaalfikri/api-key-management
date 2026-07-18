pub mod state;
pub mod app;
pub mod db;
pub mod dto;
pub mod errors;
pub mod handlers;
pub mod models;
pub mod repositories;
pub mod routes;
pub mod services;
pub mod settings;
pub mod utils;

#[tokio::main]
async  fn main() {
    println!("Hello, world!");
}
