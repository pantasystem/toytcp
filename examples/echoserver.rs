use anyhow::Result;
use std::{env, io, net::Ipv4Addr, str};
use toytcp::tcp::TCP;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let addr: Ipv4Addr = args[1].parse()?;
    let port: u16 = args[2].parse()?;
    echo_server(addr, port)?;
    Ok(())
}

fn echo_server(local_addr: Ipv4Addr, local_port: u16) -> Result<()> {
    let tcp = TCP::new();
    let listening_socket = tcp.listen(local_addr, local_port)?;
    dbg!("listeing..");
    loop {
        let connected_socket = tcp.accept(listening_socket)?;
        dbg!("accepted!", connected_socket.1, connected_socket.3);
    }
}



// 09:06:14.839398 IP 10.0.0.1.42502 > 10.0.1.1.40000: Flags [S], seq 1781165593, win 4380, length 0
// 09:06:14.839458 IP 10.0.1.1.40000 > 10.0.0.1.42502: Flags [S.], seq 1358661489, ack 1781165594, win 64240, options [mss 1460], length 0
// 09:06:14.839656 IP 10.0.0.1.42502 > 10.0.1.1.40000: Flags [.], ack 1, win 4380, length 0