use dns_starter_rust::{answer::DnsAnswer, header::DnsHeader, question::DnsQuestion, DnsQuery};
use std::net::UdpSocket;

fn main() {
    let udp_socket = UdpSocket::bind("127.0.0.1:2053").expect("Failed to bind to address");
    let mut buf = [0; 512];

    loop {
        match udp_socket.recv_from(&mut buf) {
            Ok((_, source)) => {
                let mut header = DnsHeader::from_bytes([0; 12]);
                header.set_id(1234);
                header.set_qr(1);
                header.set_qdcount(1);
                header.set_ancount(1);
                let mut questions = Vec::new();
                questions.push(DnsQuestion::new("codecrafters.io".to_string(), 1, 1));
                let mut answwers = Vec::new();
                answwers.push(DnsAnswer::new(
                    "codecrafters.io".to_string(),
                    1,
                    1,
                    60,
                    vec![8, 8, 8, 8],
                ));
                let query = DnsQuery::new(header, questions, answwers);

                let response = query.to_bytes();
                udp_socket
                    .send_to(&response, source)
                    .expect("Failed to send response");
            }
            Err(e) => {
                eprintln!("Error receiving data: {}", e);
                break;
            }
        }
    }
}
