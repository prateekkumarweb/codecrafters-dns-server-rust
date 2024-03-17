pub struct DnsHeader([u8; 12]);

impl DnsHeader {
    pub fn id(&self) -> u16 {
        u16::from_be_bytes([self.0[0], self.0[1]])
    }

    pub fn qr(&self) -> u8 {
        (self.0[2] & 0b1000_0000) >> 7
    }

    pub fn opcode(&self) -> u8 {
        (self.0[2] & 0b0111_1000) >> 3
    }

    pub fn aa(&self) -> u8 {
        (self.0[2] & 0b0000_0100) >> 2
    }

    pub fn tc(&self) -> u8 {
        (self.0[2] & 0b0000_0010) >> 1
    }

    pub fn rd(&self) -> u8 {
        self.0[2] & 0b0000_0001
    }

    pub fn ra(&self) -> u8 {
        (self.0[3] & 0b1000_0000) >> 7
    }

    pub fn z(&self) -> u8 {
        (self.0[3] & 0b0111_0000) >> 4
    }

    pub fn rcode(&self) -> u8 {
        self.0[3] & 0b0000_1111
    }

    pub fn qdcount(&self) -> u16 {
        u16::from_be_bytes([self.0[4], self.0[5]])
    }

    pub fn ancount(&self) -> u16 {
        u16::from_be_bytes([self.0[6], self.0[7]])
    }

    pub fn nscount(&self) -> u16 {
        u16::from_be_bytes([self.0[8], self.0[9]])
    }

    pub fn arcount(&self) -> u16 {
        u16::from_be_bytes([self.0[10], self.0[11]])
    }

    pub fn new() -> DnsHeader {
        DnsHeader([0; 12])
    }

    pub fn set_id(&mut self, id: u16) {
        self.0[0] = (id >> 8) as u8;
        self.0[1] = id as u8;
    }

    pub fn set_qr(&mut self, qr: u8) {
        self.0[2] |= qr << 7;
    }

    pub fn set_opcode(&mut self, opcode: u8) {
        self.0[2] |= opcode << 3;
    }

    pub fn set_aa(&mut self, aa: u8) {
        self.0[2] |= aa << 2;
    }

    pub fn set_tc(&mut self, tc: u8) {
        self.0[2] |= tc << 1;
    }

    pub fn set_rd(&mut self, rd: u8) {
        self.0[2] |= rd;
    }

    pub fn set_ra(&mut self, ra: u8) {
        self.0[3] |= ra << 7;
    }

    pub fn set_z(&mut self, z: u8) {
        self.0[3] |= z << 4;
    }

    pub fn set_rcode(&mut self, rcode: u8) {
        self.0[3] |= rcode;
    }

    pub fn set_qdcount(&mut self, qdcount: u16) {
        self.0[4] = (qdcount >> 8) as u8;
        self.0[5] = qdcount as u8;
    }

    pub fn set_ancount(&mut self, ancount: u16) {
        self.0[6] = (ancount >> 8) as u8;
        self.0[7] = ancount as u8;
    }

    pub fn set_nscount(&mut self, nscount: u16) {
        self.0[8] = (nscount >> 8) as u8;
        self.0[9] = nscount as u8;
    }

    pub fn set_arcount(&mut self, arcount: u16) {
        self.0[10] = (arcount >> 8) as u8;
        self.0[11] = arcount as u8;
    }

    pub fn to_bytes(&self) -> [u8; 12] {
        self.0
    }

    pub fn from_bytes(bytes: [u8; 12]) -> DnsHeader {
        DnsHeader(bytes)
    }
}
