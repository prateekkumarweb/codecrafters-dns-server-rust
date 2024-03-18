use answer::DnsAnswer;
use header::DnsHeader;
use question::DnsQuestion;

pub mod answer;
pub mod header;
pub mod question;

pub struct DnsPacket {
    pub header: DnsHeader,
    questions: Vec<DnsQuestion>,
    answers: Vec<DnsAnswer>,
}

impl DnsPacket {
    pub fn new(
        header: DnsHeader,
        questions: Vec<DnsQuestion>,
        answers: Vec<DnsAnswer>,
    ) -> DnsPacket {
        DnsPacket {
            header,
            questions,
            answers,
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = self.header.to_bytes().to_vec();
        for question in &self.questions {
            bytes.extend(&question.to_bytes());
        }
        for answer in &self.answers {
            bytes.extend(&answer.to_bytes());
        }
        bytes
    }

    pub fn from_bytes(bytes: &[u8]) -> Self {
        let header = DnsHeader::from_bytes([
            bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
            bytes[8], bytes[9], bytes[10], bytes[11],
        ]);
        let mut questions = Vec::new();
        let mut answers = Vec::new();
        let mut index = 12;
        for _ in 0..header.qdcount() {
            let (question, new_index) = DnsQuestion::from_bytes(bytes, index);
            questions.push(question);
            index = new_index;
        }
        for _ in 0..header.ancount() {
            let (answer, new_index) = DnsAnswer::from_bytes(bytes, index);
            answers.push(answer);
            index = new_index;
        }
        DnsPacket {
            header,
            questions,
            answers,
        }
    }
}
