use dns_starter_rust::{answer::DnsAnswer, DnsPacket};
use std::net::UdpSocket;

fn main() {
    let udp_socket = UdpSocket::bind("127.0.0.1:2053").expect("Failed to bind to address");
    let mut buf = [0; 512];

    loop {
        match udp_socket.recv_from(&mut buf) {
            Ok((size, source)) => {
                let query = DnsPacket::from_bytes(&buf[..size]);
                let mut header = query.header;
                let questions = query.questions;
                header.set_qr(1);
                header.set_aa(0);
                header.set_tc(0);
                header.set_ra(0);
                header.set_z(0);
                header.set_rcode(match header.opcode() {
                    0 => 0,
                    _ => 4,
                });
                header.set_nscount(0);
                header.set_arcount(0);
                let mut answwers = Vec::new();
                for q in questions.iter() {
                    answwers.push(DnsAnswer::new(
                        q.qname().to_owned(),
                        q.qtype(),
                        q.qclass(),
                        60,
                        vec![8, 8, 8, 8],
                    ));
                }
                header.set_ancount(answwers.len() as u16);
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
