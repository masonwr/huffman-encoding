mod encoding;
mod ioutils;
mod priorityq;

use encoding::huffman_tree;
use ioutils::{read_header, write_header, HuffmanPathReader, HuffmanPathWriter};

use std::collections::BTreeMap;
use std::io::{Cursor, Read, Seek, SeekFrom, Write};

fn main() -> anyhow::Result<()> {
    // let mut in_bytes = std::fs::File::open("moby.txt").expect("could not open file");
    let mut in_bytes = Cursor::new("is this actually working?".as_bytes());

    let mut encoded_buff: Vec<u8> = vec![];
    encode(&mut in_bytes, &mut encoded_buff)?;

    let mut encoded_data = &encoded_buff[..];
    let mut out: Vec<u8> = Vec::new();

    decode(&mut encoded_data, &mut out)?;

    let out_str = String::from_utf8(out)?;
    println!("{}", out_str);

    Ok(())
}

fn encode<R: Read + Seek>(reader: &mut R, writer: &mut impl Write) -> anyhow::Result<()> {
    let mut hist: BTreeMap<u8, usize> = BTreeMap::new();
    for byte in reader.bytes() {
        let i = byte?;
        *hist.entry(i).or_insert(0) += 1;
    }

    let root = huffman_tree(&hist)?;

    // translate tree into encoding paths.
    let symbol_table = root.to_symbol_table();
    write_header(writer, &mut hist)?;

    let mut path_writer = HuffmanPathWriter::new();

    // reset reader
    reader.seek(SeekFrom::Start(0))?;
    for b in reader.bytes() {
        match symbol_table.get(&b?) {
            Some(path) => {
                path_writer.write_path(writer, path)?;
            }
            None => anyhow::bail!("key should never be empty"),
        }
    }

    path_writer.flush(writer)?;
    Ok(())
}

fn decode(reader: &mut impl Read, writer: &mut impl Write) -> anyhow::Result<()> {
    let hist = read_header(reader)?;
    let root = huffman_tree(&hist)?;
    let message_size = *root.count();

    let mut path_reader = HuffmanPathReader::new(message_size);
    while let Some(byte) = path_reader.next_byte(reader, &root)? {
        writer.write(&[byte])?;
    }

    Ok(())
}
