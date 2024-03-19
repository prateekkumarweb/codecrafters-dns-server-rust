#[derive(Debug, Clone)]
pub struct DnsQuestion {
    qname: String,
    qtype: u16,
    qclass: u16,
}

impl DnsQuestion {
    pub fn new(qname: String, qtype: u16, qclass: u16) -> DnsQuestion {
        DnsQuestion {
            qname,
            qtype,
            qclass,
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        for part in self.qname.split('.') {
            bytes.push(part.len() as u8);
            bytes.extend(part.as_bytes());
        }
        bytes.push(0);
        bytes.extend(&self.qtype.to_be_bytes());
        bytes.extend(&self.qclass.to_be_bytes());
        bytes
    }

    pub fn from_bytes(bytes: &[u8], index: usize) -> (DnsQuestion, usize) {
        let mut qname_parts = Vec::new();
        let mut index = index;
        loop {
            let b1 = bytes[index];
            if b1 & 0b1100_0000 == 0b1100_0000 {
                let b2 = bytes[index + 1];
                let offset = u16::from_be_bytes([b1 & 0b0011_1111, b2]);
                let mut offset = offset as usize;
                loop {
                    let len = bytes[offset] as usize;
                    if len == 0 {
                        break;
                    }
                    qname_parts.push(
                        std::str::from_utf8(&bytes[offset + 1..offset + 1 + len])
                            .expect("Failed to parse qname"),
                    );
                    offset += len + 1;
                }
                index += 2;
                break;
            }
            let len = bytes[index] as usize;
            if len == 0 {
                index += 1;
                break;
            }
            qname_parts.push(
                std::str::from_utf8(&bytes[index + 1..index + 1 + len])
                    .expect("Failed to parse qname"),
            );
            index += len + 1;
        }
        let qtype = u16::from_be_bytes([bytes[index], bytes[index + 1]]);
        let qclass = u16::from_be_bytes([bytes[index + 2], bytes[index + 3]]);
        index += 4;
        (
            DnsQuestion::new(qname_parts.join("."), qtype, qclass),
            index,
        )
    }

    pub fn qname(&self) -> &str {
        &self.qname
    }

    pub fn qtype(&self) -> u16 {
        self.qtype
    }

    pub fn qclass(&self) -> u16 {
        self.qclass
    }
}
