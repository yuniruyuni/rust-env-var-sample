use std::env;

fn main() {
   let host = env::var("HOST").unwrap_or(String::from("localhost"));
   let port: u16 = env::var("PORT").unwrap_or(String::from("8080")).parse().unwrap();
   println!("{}:{}", host, port);
}
