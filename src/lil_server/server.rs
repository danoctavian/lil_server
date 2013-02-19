use std::*;

pub fn start() {
  info!("started lil_server");
//  debug!(fmt!("a const %d", 1));
  io::println(fmt!("a const %d", sort::MIN_GALLOP as int));
  let taskBuilder : task::TaskBuilder = task::task(); 
  //let ioTask : uv_iotask::IoTask = uv_iotask::spawn_iotask(taskBuilder);

  do task::spawn {
    debug!("started solving task");    
  }
  let (port, chan) : (pipes::Port<net_tcp::TcpNewConnection>,
                    pipes::Chan<net_tcp::TcpNewConnection>) = pipes::stream();
  let (bport, bchan) : (pipes::Port<int>, pipes::Chan<int>) = pipes::stream();

   task::spawn_sched(task::ManualThreads(1u), || {
      debug!("connection handler started");
      let newConn = port.recv();
      debug!("handling new conn");

      let mut acceptRes = net_tcp::accept(newConn);
      match acceptRes {
        result::Ok(tcpSocket) => {
 //         let socketBuf = net_tcp::socket_buf(tcpSocket);
          debug!("accepted connection");
          let bytes = tcpSocket.read(1u);
          debug!("read byte");
          
          bchan.send(1);
//          socketBuf.write(str::to_bytes("die mothafucka"));
          debug!("wrote to socket");
        }
        result::Err(_) => error!("failed to accept socket connection")
      };
      debug!("accepted connection");

    });


  do task::spawn {

    let ipAddr = net_ip::v4::parse_addr(~"127.0.0.1");
    let ioTask =  uv::global_loop::get();
    let servPort = 6969;
    debug!("started ioTask");

    net_tcp::listen(ipAddr, servPort, 128, ioTask,
                   on_establish_cb,
                   |newConn, kill_chan| {
                     debug!("new conn");

                     chan.send(newConn);

      //               bport.recv();
                       /*
                     task::spawn_sched (task::ManualThreads(1u), || {
                       debug!("this is the spawned task running");
                       let tcpSocket = result::unwrap(acceptRes);

                       let res = tcpSocket.read_start();
                       */

/*
                       let mut acceptRes = net_tcp::accept(newConn);
                       match acceptRes {
                         result::Ok(tcpSocket) => {
                           let socketBuf = net_tcp::socket_buf(tcpSocket);
                           debug!("accepted connection");
                           socketBuf.read_byte();
                           debug!("read byte");
                           socketBuf.write(str::to_bytes("die mothafucka"));
                           debug!("wrote to socket");
                         }
                         result::Err(_) => error!("failed to accept socket connection")
                       };
 
                     });

                       */
                   } 
                );
  }
                   
}

fn on_establish_cb(chan : oldcomm::Chan<Option<net_tcp::TcpErrData>>) {
  debug!("etsablished listener");
  do task::spawn {
    debug!("spawned task in establishment");
  };
}

fn new_connect_cb(newConn : net_tcp::TcpNewConnection,
                  kill_chan : oldcomm::Chan<Option<net_tcp::TcpErrData>>)
                   {
//  return result::Ok(());
  io::println("new connection arrived");
  debug!("connect cb");
//  let cont_po = oldcomm::Port<option::Option<net_tcp::TcpErrData>>();
/*
  let cont_po = oldcomm::Port();
  let cont_ch = oldcomm::Chan(cont_po);
*/
     /*
    let mut acceptRes = net_tcp::accept(newConn);
    match acceptRes {
      result::Ok(tcpSocket) => {

        let socketBuf = net_tcp::socket_buf(tcpSocket);
        debug!("accepted connection");
        //socketBuf.write(str::to_bytes("die mothafucka"));
        socketBuf.read_byte();
        debug!("wrote to socket");
        debug!("got result of writing");
      }
      result::Err(_) => error!("failed to accept socket connection")
    };
    */
    //oldcomm::send(cont_ch, true);
  debug!("spawned a task");
 // oldcomm::recv(cont_po);
/*
  match oldcomm::recv(cont_po) {
    // shut down listen()
    some(err_data) => { oldcomm::send(kill_chan, some(err_data)) }
    // wait for next connection
    none => {}
  }
*/
}
