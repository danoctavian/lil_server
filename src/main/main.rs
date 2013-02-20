//use lil_server::*;
use std::*;

fn main() {
 let conf = ~lil_server::Config {
   host: ~"127.0.0.1",
   files_root: ~"./data/resources/"
 };
 lil_server::start(conf);
}
