use std::convert::TryInto;
use std::io::prelude::*;
use std::mem::size_of;
use std::{borrow::BorrowMut, collections::BTreeMap};

use crate::encoding::Direction;

#[derive(Debug)]
pub struct HuffmanPathWriter {
    bit_count: u8,
    buffer: u8,
}

impl HuffmanPathWriter {
    pub fn new() -> Self {
        HuffmanPathWriter {
            bit_count: 0,
            buffer: 0,
        }
    }

    pub fn write_path(
        &mut self,
        writer: &mut impl Write,
        path: &Vec<Direction>,
    ) -> anyhow::Result<()> {

        for comp in path {
            self.write_path_comp(writer, comp)?
        }

        Ok(())
    }

    fn write_path_comp(
        &mut self,
        writer: &mut impl Write,
        direc: &Direction,
    ) -> anyhow::Result<()> {
        println!("direc: {:?}", direc);
        // flip left bits on, leave right bits off

        match direc {
            Direction::Left => self.buffer |= (1 << self.bit_count),
            Direction::Right => {
                // self.buffer = self.buffer << 1;
                ()
                // self.buffer = self.buffer << 1;
            }
        };

        println!("buffer: {:#010b}", self.buffer);
        println!("count: {}", self.bit_count);

        self.bit_count += 1;

        if self.bit_count == 8 {
            self.flush(writer);
        }

        Ok(())
    }

    pub fn flush(&mut self, writer: &mut impl Write) -> anyhow::Result<()> {
        if self.bit_count == 0 {
            return Ok(());
        }

        writer.write(&vec![self.buffer])?;

        self.buffer = 0;
        self.bit_count = 0;

        Ok(())
    }
}

pub fn write_header(writer: &mut impl Write, hist: &BTreeMap<u8, usize>) -> anyhow::Result<()> {
    writer.write(&[hist.len() as u8])?;

    for (k, v) in hist {
        let mut out = vec![*k];
        out.extend(v.to_be_bytes());
        writer.write(&out)?;
    }

    Ok(())
}

pub fn read_header(mut reader: impl Read) -> anyhow::Result<(BTreeMap<u8, usize>, impl Read)> {
    let mut size_buffer = vec![0; 1]; // just get fist byte
    reader.read(&mut size_buffer)?;
    let hist_size = *size_buffer.first().expect("file to not be empty");

    let usize_size = size_of::<usize>();
    let mut buffer = vec![0; 1 + usize_size]; // sized for (k,v) pair (1 byte + usize)
    let mut hist = BTreeMap::new();
    for _ in 0..hist_size {
        reader.read(&mut buffer)?;

        let val = buffer[0];
        let count = usize::from_be_bytes(buffer[1..].try_into().expect("incorrect byte lenght"));

        hist.insert(val, count);
    }

    Ok((hist, reader))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_path_writer() -> anyhow::Result<()> {
        use Direction::*;

        let mut path_writer = HuffmanPathWriter::new();

        let mut buffer: Vec<u8> = vec![];

        path_writer.write_path(&mut buffer, &vec![Left, Right, Left, Right])?;
        path_writer.flush(&mut buffer)?;

        assert_eq!(buffer[0], 0b00000101);

        Ok(())
    }

    #[test]
    fn header_o_empty() -> anyhow::Result<()> {
        let mut buffer: Vec<u8> = vec![];

        let hist = BTreeMap::new();

        write_header(&mut buffer, &hist)?;

        assert_eq!(buffer, vec![0]);

        Ok(())
    }

    #[test]
    fn header_o() -> anyhow::Result<()> {
        let mut buffer: Vec<u8> = vec![];

        let mut hist = BTreeMap::new();

        hist.insert(97, 5);
        hist.insert(98, 8);

        write_header(&mut buffer, &hist)?;

        // size of hist
        assert_eq!(buffer[0..1], vec![2]);

        // input_buf
        // let mut i_buf: Vec<u8> = vec![];
        let read_hist = read_header(&buffer[..])?;

        assert_eq!(read_hist.len(), 2);
        assert_eq!(read_hist.get(&97), Some(&5));
        assert_eq!(read_hist.get(&98), Some(&8));

        Ok(())
    }
}
