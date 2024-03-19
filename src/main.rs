use dns_starter_rust::{answer::DnsAnswer, header::DnsHeader, DnsPacket};
use rand::random;
use std::net::UdpSocket;

fn main() {
    let udp_socket = UdpSocket::bind("127.0.0.1:2053").expect("Failed to bind to address");
    let mut buf = [0; 512];

    let args = std::env::args().collect::<Vec<String>>();
    let mut resolver = None;
    if args[1] == "--resolver" {
        resolver = Some(&args[2]);
    }

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
                    if let Some(resolver) = &resolver {
                        let id = random::<u16>();
                        let mut qheader = DnsHeader::new();
                        qheader.set_id(id);
                        qheader.set_qr(0);
                        qheader.set_opcode(header.opcode());
                        qheader.set_rd(header.rd());
                        qheader.set_qdcount(1);

                        let query = DnsPacket::new(qheader, vec![q.clone()], vec![]);

                        udp_socket
                            .send_to(query.to_bytes().as_ref(), resolver)
                            .expect("Failed to send query");

                        let (size, _) = udp_socket
                            .recv_from(&mut buf)
                            .expect("Failed to receive response");
                        let response = DnsPacket::from_bytes(&buf[..size]);
                        assert!(response.header.id() == id);
                        answwers.extend(response.answers);
                    } else {
                        answwers.push(DnsAnswer::new(
                            q.qname().to_owned(),
                            q.qtype(),
                            q.qclass(),
                            60,
                            vec![8, 8, 8, 8],
                        ));
                    }
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
