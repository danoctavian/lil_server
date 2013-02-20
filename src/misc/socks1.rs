extern mod std;

use std::*;
        use net::tcp;
        use net::ip;
        use cell::Cell;
        use net::tcp::TcpSocket;
        use uv;
/*
  type ReaderPortFactory<U> =
        ~fn(TcpSocketBuf) -> FlatPort<int, U, ReaderBytePort<TcpSocketBuf>>;
    type WriterChanFactory<F> =
        ~fn(TcpSocketBuf) -> FlatChan<int, F, WriterByteChan<TcpSocketBuf>>;
*/
fn main() {
  
  io::println("running main");
  test_some_tcp_stream(6969);
}
/*
   fn test_some_tcp_stream<U: flatpipes::Unflattener<int>,
                          F: flatpipes::Flattener<int>>(
       reader_port: ReaderPortFactory<U>,
        writer_chan: WriterChanFactory<F>,
*/
fn test_some_tcp_stream (
        port: uint) {
        // Indicate to the client task that the server is listening
        let (begin_connect_port, begin_connect_chan) = pipes::stream();
        // The connection is sent from the server task to the receiver task
        // to handle the connection
        //let (accept_port, accept_chan) = pipes::stream();
        // The main task will wait until the test is over to proceed
//        let (finish_port, finish_chan) = pipes::stream();

        let addr0 = ip::v4::parse_addr("127.0.0.1");

        let begin_connect_chan = Cell(begin_connect_chan);
        //let accept_chan = Cell(accept_chan);

        // The server task
        let addr = copy addr0;
        do task::spawn || {
            let iotask = &uv::global_loop::get();
            let begin_connect_chan = begin_connect_chan.take();
            //let accept_chan = accept_chan.take();
            let listen_res = do tcp::listen(
                copy addr, port, 128, *iotask, |_kill_ch| {
                    // Tell the sender to initiate the connection
                    io::println("listening");
                    begin_connect_chan.send(())
                }) |new_conn, kill_ch| {

                // Incoming connection. Send it to the receiver task to accept
                let (res_port, res_chan) = pipes::stream();
//                accept_chan.send((new_conn, res_chan));
                 task::spawn_sched(task::ManualThreads(1u), || {
                 io::println("spawned task");
                 let accept_result = tcp::accept(new_conn);
                 io::println("accepted");
                 assert accept_result.is_ok();
                 let sock = result::unwrap(accept_result);
                 res_chan.send(());
                 
                 let rd = sock.read(0u);
                 io::println(str::from_bytes(result::unwrap(rd)));
                 sock.write(str::to_bytes("my resp 3333333333333333333333333333333333333333333333333333333"));
                 //finish_chan.send(());

               });
               // Wait until the connection is accepted
                res_port.recv();

                // Stop listening
               // kill_ch.send(None)
            };

            assert listen_res.is_ok();
        }

        // Client task
        let addr = copy addr0;

        if (false) {
        do task::spawn || {

            // Wait for the server to start listening
            begin_connect_port.recv();

            io::println("connecting");
            let iotask = &uv::global_loop::get();
            let connect_result = tcp::connect(copy addr, port, *iotask);
            assert connect_result.is_ok();
            let sock = result::unwrap(connect_result);
            let socket_buf: tcp::TcpSocketBuf = tcp::socket_buf(sock);

            // TcpSocketBuf is a Writer!
//            let chan = writer_chan(socket_buf);

/*
            for int::range(0, 10) |i| {
                io::println("sending %?", i);
                chan.send(i)
            }
*/
        }
        }
        // Reciever task
/*
        do task::spawn || {
            // Wait for a connection
            let (conn, res_chan) = accept_port.recv();

            io::println("accepting connection");
            let accept_result = tcp::accept(conn);
            io::println("accepted");
            assert accept_result.is_ok();
            let sock = result::unwrap(accept_result);
            res_chan.send(());
            
            let rd = sock.read(0u);
            io::println(str::from_bytes(result::unwrap(rd)));
//            let socket_buf: tcp::TcpSocketBuf = tcp::socket_buf(sock);
            // TcpSocketBuf is a Reader!
//            let port = reader_port(socket_buf);
            for int::range(0, 10) |i| {
                let j = port.recv();
                io::println("receieved %?", j);
                assert i == j;
            }
            // The test is over!
        }
}
*/
//        finish_port.recv();
    }

