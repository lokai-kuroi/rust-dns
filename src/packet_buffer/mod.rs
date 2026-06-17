use anyhow::{bail, Result};

pub struct BytePacketBuffer {
    pub buffer: [u8;512],
    pub reader_pos: usize,
}

impl BytePacketBuffer {

    pub fn new() -> BytePacketBuffer {
        BytePacketBuffer {
            buffer: [0;512],
            reader_pos: 0,
        }
    }

    // Sets the read pointer to a specific position, result in an Error if its out of bound
    // fn seek
    fn set_reader_pos(&mut self, pos: usize) -> Result<()>{
        // TODO Change to not magic number
        if pos > 512 {
            eprintln!("Cannot set new reader positon, will result out of bound"); // TODO Change to
            bail!("Cannot set new reader positon, will result out of bound");
        }
        self.reader_pos = pos;
        Ok(())
    }

    // Get Current Positon
    fn get_cur_pos(&self) -> usize {
        self.reader_pos
    }

    // Moving the read pointer positon by x steps
    pub fn step_positon_pointer(&mut self, steps: usize) -> Result<()> {
        self.reader_pos += steps;
        if self.reader_pos > 512 {
            eprintln!("Change in {} steps resulted in a invalid pointer. Reset pointer to last state", steps);
            self.reader_pos -= steps;
            bail!("Change in {} steps resulted in a invalid pointer. Reset pointer to last state", steps);
        }
        Ok(())
    }

    // Read Single Bytes from the buffer and move the position of the pointer
    fn read(&mut self) -> Result<u8> {
        if self.reader_pos >= 512 {
            eprintln!("Read Position out of bounds");
            bail!("Read Position out of Bounds");
        }
        let reader_pos: usize = self.reader_pos;
        self.reader_pos += 1;
        Ok(self.buffer[reader_pos])
    }

    // Read from buffer without changing the position pointer
    fn get(&mut self) -> Result<u8> {
        if self.reader_pos >= 512 {
            eprintln!("Read Position out of Bounds");
            bail!("Read Position out of Bounds");
        }
        Ok(self.buffer[self.reader_pos])
    }

    fn get_range(&mut self, start: usize, len: usize) -> Result<&[u8]> {
        if start + len >= 512 {
            eprintln!("Cant get range out of bound");
            bail!("Cant get range out of bound");
        }
        Ok(&self.buffer[start..start + len as usize])
    }

    fn read_u16(&mut self) -> Result<u16> {
        let res = ((self.read()? as u16) << 8) | (self.read()? as u16);
        Ok(res)
    }

    fn read_u32(&mut self) -> Result<u32> {
        let res = ((self.read()? as u32) << 24) | ((self.read()? as u32) << 16) | ((self.read()? as u32) << 8) | ((self.read()? as u32) << 0);
        Ok(res)
    }

    // Read qname
    // TODO To Implement
    fn read_qname(&mut self, outstr: &mut String) -> Result<()> {
        Ok(())
    }
}
