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
}
