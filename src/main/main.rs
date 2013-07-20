use lil_server::*;
use std::*;

#[main]
fn main() {
 let conf = ~Config {
   host: ~"127.0.0.1",
   files_root: ~"./data/resources/"
 };
  println("running server");

  match get_static_file("data/resources/", "hello.html") {
    Ok(data) => {
      println(str::from_bytes(data)); 
    }
    Err(err) => {
      println(err);
    }
  }
 server::start(conf);
}
