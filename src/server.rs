use tokio::net::{UdpSocket, TcpStream};

pub const MOST_COMMON_PORTS_1002: &[u16] = &[
    5601, 9300, 80, 23, 443, 21, 22, 25, 3389, 110, 445, 139, 143, 53, 135, 3306, 8080, 1723, 111,
    995, 993, 5900, 1025, 587, 8888, 199, 1720, 465, 548, 113, 81, 6001, 10000, 514, 5060, 179,
    1026, 2000, 8443, 8000, 32768, 554, 26, 1433, 49152, 2001, 515, 8008, 49154, 1027, 5666, 646,
    5000, 5631, 631, 49153, 8081, 2049, 88, 79, 5800, 106, 2121, 1110, 49155, 6000, 513, 990, 5357,
    427, 49156, 543, 544, 5101, 144, 7, 389, 8009, 3128, 444, 9999, 5009, 7070, 5190, 3000, 5432,
    1900, 3986, 13, 1029, 9, 5051, 6646, 49157, 1028, 873, 1755, 2717, 4899, 9100, 119, 37, 1000,
    3001, 5001, 82, 10010, 1030, 9090, 2107, 1024, 2103, 6004, 1801, 5050, 19, 8031, 1041, 255,
];

#[derive(Clone)]
pub struct Server {
    buf: Vec<u8>,
    addr: String,
}

impl Server {
    pub fn new(buf: Vec<u8>, addr: &str) -> Self {
        Self { buf: buf, addr: addr.to_owned()} 
    }

    pub async fn run(&self, socket: &UdpSocket, port: u16) {
        socket.connect((self.addr.trim(), port)).await.expect("Could not connect");
        loop {
            match socket.send(&self.buf).await {
                Ok(_) => (),
                Err(_) => (),
            };
            
        }
    }

    pub async fn scan_port(&self, addr: &str) -> Vec<u16>{
        let mut open_ports: Vec<u16> = Vec::new();
        for port in MOST_COMMON_PORTS_1002 {
            match TcpStream::connect((addr, port.to_owned())).await {
                Ok(_) => open_ports.push(port.to_owned()),
                Err(_) => (),
            }
        }
        open_ports
    }   
}
