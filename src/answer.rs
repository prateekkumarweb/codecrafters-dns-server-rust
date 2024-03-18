pub struct DnsAnswer {
    name: String,
    qtype: u16,
    qclass: u16,
    ttl: u32,
    rdlength: u16,
    rdata: Vec<u8>,
}

impl DnsAnswer {
    pub fn new(name: String, qtype: u16, qclass: u16, ttl: u32, rdata: Vec<u8>) -> DnsAnswer {
        DnsAnswer {
            name,
            qtype,
            qclass,
            ttl,
            rdlength: rdata.len() as u16,
            rdata,
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        for part in self.name.split('.') {
            bytes.push(part.len() as u8);
            bytes.extend(part.as_bytes());
        }
        bytes.push(0);
        bytes.extend(&self.qtype.to_be_bytes());
        bytes.extend(&self.qclass.to_be_bytes());
        bytes.extend(&self.ttl.to_be_bytes());
        bytes.extend(&self.rdlength.to_be_bytes());
        bytes.extend(&self.rdata);
        bytes
    }

    pub fn from_bytes(bytes: &[u8], index: usize) -> (DnsAnswer, usize) {
        let mut name_parts = Vec::new();
        let mut index = index;
        loop {
            let len = bytes[index] as usize;
            if len == 0 {
                index += 1;
                break;
            }
            name_parts.push(
                std::str::from_utf8(&bytes[index + 1..index + 1 + len])
                    .expect("Failed to parse name"),
            );
            index += len + 1;
        }
        let qtype = u16::from_be_bytes([bytes[index], bytes[index + 1]]);
        let qclass = u16::from_be_bytes([bytes[index + 2], bytes[index + 3]]);
        let ttl = u32::from_be_bytes([
            bytes[index + 4],
            bytes[index + 5],
            bytes[index + 6],
            bytes[index + 7],
        ]);
        let rdlength = u16::from_be_bytes([bytes[index + 8], bytes[index + 9]]);
        let rdata = bytes[index + 10..index + 10 + rdlength as usize].to_vec();
        index += 10 + rdlength as usize;
        (
            DnsAnswer::new(name_parts.join("."), qtype, qclass, ttl, rdata),
            index,
        )
    }
}
