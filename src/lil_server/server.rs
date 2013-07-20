use config::*;
use extra::{net, uv, uv_global_loop};
use std::*;
use std::option::*;
use std::to_bytes::ToBytes;

pub fn start(config: &Config) {

  use extra::net::tcp::*;
  let addr = net::ip::v4::parse_addr(config.host);
  let port = 6969;

  let iotask = uv_global_loop::get();
  let stack_config = (*config).clone(); 
  do listen(addr, port, 64, &iotask,
    |kill_ch| {
      // pass the kill_ch to your main loop or wherever you want
      // to be able to externally kill the server from
    })
    // this callback is ran when a new connection arrives
    |new_conn, kill_ch| {
      let (cont_po, cont_ch) = comm::stream::<Option<TcpErrData>>();

      let config_cpy = stack_config.clone();
      do task::spawn {
          let accept_result = accept(new_conn);
          match accept_result {
              Err(accept_error) => {
                  cont_ch.send(Some(accept_error));
              },
              Ok(sock) => {
                 cont_ch.send(None);
                 handle_request(&config_cpy, &sock);
              }
          }
      };
      match cont_po.recv() {
        // shut down listen()
        Some(err_data) => kill_ch.send(Some(err_data)),
        // wait for next connection
        None => ()
      }
    };

}

fn handle_request(conf : &Config, socket : &net::tcp::TcpSocket) {
  println("in handle request func");
  let mut response = ("404 not found").to_bytes(false);
  match socket.read(0u) {
    Ok(req_bytes) => {
      let req_text = str::from_bytes(req_bytes);
      //io::println(req_text);
      match get_request_content(req_text) {
        Ok(req) => {
          match get_static_file(conf.files_root, req.file_name)  {
            Ok(content) => {
              let header = ~"HTTP/1.0 200 OK\r\nContent-Type: text/html\r\n\r\n";
              response = vec::append(header.to_bytes(false), content);
            }
            Err(err_data) => { println("could not read file");}
          }
        }
        Err(err_data) => {
          println("request does not have the correct format");
         }
        }
    }
    Err(crap) => {
      debug!("read failed");
    }
  };
  socket.write(response);
}

struct Request {
  file_name: ~str
}

pub fn get_static_file(root_str : &str, file_name : &str)
                  -> Result<~[u8], ~str> {

  println("getting a static file");
  println(file_name);
  let path_str = root_str.to_owned().append(file_name);
  let full_path = 
   path::Path(path_str);
  io::println(full_path.to_str());
  println("computed full path");

  if (is_valid_path(file_name)) {
    println("valid path");

    match io::read_whole_file
            (~full_path)  {
       Ok(content) => Ok(content),
       Err(err_data) => Err(~"wrong file")
    }
  } else { Err(~"wrong file")}
}

fn is_valid_path(path : &str) -> bool {
  match path.find_str("..") {
    Some(x) => false,
    None => true
  }
}


pub fn get_request_content(req_text : &str) -> Result<~Request, ~str>   {
  let words : ~[&str] = req_text.word_iter().collect();
  if (words.len() >= 2 && words[0] == "GET") {
    Ok(~Request{file_name: words[1].to_owned()})
  } else { Err(~"wrong request format") } 
}

