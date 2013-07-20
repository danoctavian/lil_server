extern mod std;
use std::getopts::*;
use std::net::tcp;
use std::net::ip;
use std::task;
use std::uv;
use core::comm;

struct C {
  pub s: ~str
}

struct D {
  pub s: @str,
  pub i: int
}

fn takestr(s : &str) {
  let c = C{s: str::from_slice(s)};
//  let c1 = C{s: ~"shit son"};
}

fn main() {
  io::println("crap son");
  let s = ~"hot";
  takestr(s);
  str::to_bytes("shit son");        
}
