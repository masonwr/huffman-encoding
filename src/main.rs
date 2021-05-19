mod encoding;
mod ioutils;
mod priorityq;

use encoding::EncodingNode;
use ioutils::{read_header, write_header, HuffmanPathReader, HuffmanPathWriter};
use priorityq::PriorityQ;

use std::collections::BTreeMap;
use std::io::Read;

// use crate::encoding::Direction;

fn main() -> anyhow::Result<()> {
    // let mut input_file = std::fs::File::open("moby.txt").expect("could not open file");
    // let mut in_buffer: Vec<u8> = vec![];
    // input_file.read_to_end(&mut in_buffer)?;

    let in_buffer = "looks like it is working...".bytes();

    // Make byte histogram (this could simply be an array too)
    let mut hist: BTreeMap<u8, usize> = BTreeMap::new();
    for byte in in_buffer.clone() {
        let i = byte;
        *hist.entry(i).or_insert(0) += 1;
    }

    // map histogram to huffman tree nodes, and build priority que
    let mut iter = hist.iter();
    let (k, v) = iter.next().expect("at leat one element");
    let mut queue = PriorityQ::new(EncodingNode::new_leaf(*k, *v));

    for (k, v) in iter {
        let node = EncodingNode::new_leaf(*k, *v);
        queue = queue.enque(node);
    }

    // Loop throught priority que, combining nodes to form tree
    let root = queue.reduce();

    let st = root.to_symbol_table();

    let mut out_buffer: Vec<u8> = vec![];
    write_header(&mut out_buffer, &hist)?;

    let mut path_writer = HuffmanPathWriter::new();

    for b in in_buffer {
        match st.get(&b) {
            Some(path) => {
                path_writer.write_path(&mut out_buffer, path)?;
            }
            None => anyhow::bail!("key should never be empty"),
        }
    }
    path_writer.flush(&mut out_buffer)?;

    let mut out_buffer = &out_buffer[..];
    // DECODE
    let _st = read_header(&mut out_buffer)?;
    // TODO parse encoding node from symboltable

    let mut path_reader = HuffmanPathReader::new(*root.count());

    let mut out = vec![];
    while let Some(byte) = path_reader.next_byte(&mut out_buffer, &root)? {
        out.push(byte);
    }

    let out_str = String::from_utf8(out)?;
    println!("{}", out_str);

    Ok(())
}
