use futures::future::{join_all};
use tokio::net::{UdpSocket};
use std::{io::stdin, net::{SocketAddr}};
mod cli;
mod server;
use server::Server;
use cli::Cli;









#[tokio::main()]
async fn main() {
    let cli = Cli::new();
    cli.ascii();
    println!("{}Please enter an address. E.g 127.0.0.1{}", &cli.green, &cli.reset);
    let mut addr = String::new();
    stdin().read_line(&mut addr).expect("Could not read input");
    let socket = UdpSocket::bind("0.0.0.0:3000".parse::<SocketAddr>().unwrap()).await.expect("Could not bind");
    let server = Server::new( vec![255; 65507], &addr.clone());
    let ports = server.scan_port(addr.trim()).await;
    println!("{}Found ports {}{ports:?}{}", &cli.green, &cli.red, &cli.reset);
    let mut servers: Vec<_> = Vec::new();
    for port in ports {
        servers.push(server.run(&socket, port));
        println!("{}Began sending packets to {}{}:{} {}",&cli.green, &cli.red, addr.trim(), port, &cli.reset);
    }
    join_all(servers).await;
}   

