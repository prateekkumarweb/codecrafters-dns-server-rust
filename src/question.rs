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
}
