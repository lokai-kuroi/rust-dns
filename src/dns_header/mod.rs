use crate::result_code::ResultCode;

#[derive(Clone, Debug)]
pub struct DnsHeader {
    pub id: u16, // 16 bits

    pub recursion_desired: bool, // 1 bit
    pub truncated_message: bool, // 1 bit
    pub authoritative_answer: bool, // 1 bit
    pub opcode: u8, // 4 bits
    pub response: bool, // 1 bit
    
    pub rescode: ResultCode, // 4 bits
    pub checking_disabled: bool, // 1 bit
    pub authed_data: bool, // 1 bit
    pub z: bool, // 1 bit 
    pub recursion_available: bool, // 1 bit

    pub question: u16, // 16 bits
    pub answers: u16, // 16 bits
    pub authoriative_entries: u16, // 16 bits
    pub resource_entries: u16, // 16 bits
}

impl DnsHeader {
    // TODO to implement
    pub fn new() -> DnsHeader {
        todo!()
    }
}
