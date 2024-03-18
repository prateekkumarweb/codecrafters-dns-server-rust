use dns_starter_rust::{answer::DnsAnswer, question::DnsQuestion, DnsPacket};
use std::net::UdpSocket;

fn main() {
    let udp_socket = UdpSocket::bind("127.0.0.1:2053").expect("Failed to bind to address");
    let mut buf = [0; 512];

    loop {
        match udp_socket.recv_from(&mut buf) {
            Ok((size, source)) => {
                let query = DnsPacket::from_bytes(&buf[..size]);
                let mut header = query.header;
                header.set_qr(1);
                header.set_aa(0);
                header.set_tc(0);
                header.set_ra(0);
                header.set_z(0);
                header.set_rcode(match header.opcode() {
                    0 => 0,
                    _ => 4,
                });
                header.set_qdcount(1);
                header.set_ancount(1);
                header.set_nscount(0);
                header.set_arcount(0);
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
                let query = DnsPacket::new(header, questions, answwers);

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
