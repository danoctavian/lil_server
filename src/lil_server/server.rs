use std::*;
use core::vec::*;
use core::result::*;
use core::str::*;

pub fn start(config: &Config) {
  info!("started lil_server instance");
  let config_cpy = copy *config;
  do task::spawn {
    let ip_addr = net_ip::v4::parse_addr(config_cpy.host);
    let iotask =  uv::global_loop::get();
    let port = 6969;
    net_tcp::listen(ip_addr, port, 128, iotask,
                   on_establish_cb,
    |new_conn, kill_chan| {
      debug!("new connection arrived");
      let (accept_port, accept_chan) = pipes::stream();
      // is spawning a task here potentially dangerous? 
      do task::spawn_sched(task::ManualThreads(2)) || {
        debug!("spawned request handler");
        match net::tcp::accept(new_conn) {
          Ok(socket) => {
            debug!("accepted succesfully");
            accept_chan.send(());
            handle_request(&config_cpy, &socket);
          } 
          Err(err_data) => {
             accept_chan.send(());
             error!("failed to accept connection");
          } 
        };
      };
      // wait for socket to be accepted
      accept_port.recv();
    });
    }                  
    do task::spawn || {
      debug!("some other random task");
    }
}

fn handle_request(conf : &Config, socket : &net::tcp::TcpSocket) {
  debug!("in handle request func");
  let mut response = str::to_bytes("404 not found");
  match socket.read(0u) {
    Ok(req_bytes) => {
      let req_text = str::from_bytes(req_bytes);
      debug!("received req");
      //io::println(req_text);
      match get_request_content(req_text) {
        Ok(req) => {
          response = str::to_bytes("your req was correct");
          match get_static_file(copy conf.files_root, req.file_name)  {
            Ok(content) => {response = content;}
            Err(err_data) => { debug!("could not read file");}
          }
        }
        Err(err_data) => {
          debug!("request does not have the correct format");
         }
//          response = str::to_bytes("your req was correct");
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

fn get_static_file(root_str : ~str, file_name : &str)
                  -> Result<~[u8], ~str> {
  debug!("getting a static file");
  io::println(file_name);
  let root_str2 = copy root_str;
  debug!("got root str");
  let path_str = str::append(root_str, file_name);
  debug!("got path string");
  let full_path = //path::Path(~"wtf");
   path::Path(path_str);
//  path::Path(str::append(~"wtf", ~"loLawgawg"));
  io::println(full_path.to_str());
  debug!("computed full path");

  if (is_valid_path(file_name)) {
    debug!("valid path");

    match io::read_whole_file
            (~full_path)  {
       Ok(content) => Ok(content),
       Err(err_data) => Err(~"wrong file")
    }
  } else { Err(~"wrong file")}
}

/* checks if it contains sneaky tricks */
fn is_valid_path(path : &str) -> bool {
  !contains(path, ~"..")
}

fn get_request_content(req_text : &str) -> Result<~Request, ~str>   {
  let words = str::words(req_text);
  if (vec::len(words) >= 2 && to_lower(words[0]) == ~"get") {
    debug!("RUST MY ASS ");
    Ok(~Request{file_name:copy words[1]})
  } else { Err(~"wrong request format") } 
}

fn on_establish_cb(chan : oldcomm::Chan<Option<net_tcp::TcpErrData>>) {
  debug!("etsablished listener");
  do task::spawn {
    debug!("spawned task in establishment");
  };
}

fn new_connect_cb(newConn : net_tcp::TcpNewConnection,
              kill_chan : oldcomm::Chan<Option<net_tcp::TcpErrData>>) {
}

