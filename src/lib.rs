use answer::DnsAnswer;
use header::DnsHeader;
use question::DnsQuestion;

pub mod answer;
pub mod header;
pub mod question;

pub struct DnsQuery {
    header: DnsHeader,
    questions: Vec<DnsQuestion>,
    answers: Vec<DnsAnswer>,
}

impl DnsQuery {
    pub fn new(
        header: DnsHeader,
        questions: Vec<DnsQuestion>,
        answers: Vec<DnsAnswer>,
    ) -> DnsQuery {
        DnsQuery {
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
}
