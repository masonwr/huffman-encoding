mod encoding;
mod ioutils;
mod priorityq;

use encoding::EncodingNode;
use ioutils::{read_header, write_header, HuffmanPathWriter};
use priorityq::PriorityQ;

use std::io::Read;
use std::{collections::BTreeMap, path};

use crate::encoding::Direction;

fn main() -> anyhow::Result<()> {
    let mut input_file = std::fs::File::open("moby.txt").expect("could not open file");

    let mut in_buffer: Vec<u8> = vec![];
    input_file.read_to_end(&mut in_buffer)?;

    let in_buffer = "aaacbb".bytes();

    println!("in_buf.len {}", in_buffer.len());

    // Make byte histogram (this could simply be an array too)
    let mut hist: BTreeMap<u8, usize> = BTreeMap::new();
    for byte in in_buffer.clone() {
        let i = byte;
        *hist.entry(i).or_insert(0) += 1;
    }

    println!();

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
    println!("{:#?}", root);

    let st = root.to_symbol_table();

    let mut out_buffer: Vec<u8> = vec![];
    write_header(&mut out_buffer, &hist)?;

    let mut path_writer = HuffmanPathWriter::new();

    for b in in_buffer {
        match st.get(&b) {
            Some(path) => {
                println!("{}:{:?}", b as char, path);
                path_writer.write_path(&mut out_buffer, path)?;
            }
            None => anyhow::bail!("key should never be empty"),
        }
    }

    path_writer.flush(&mut out_buffer)?;

    let mut out_buffer = &out_buffer[..];
    // DECODE
    let (st, mut out_buffer) = read_header(out_buffer)?;

    let mut new_buf: Vec<u8> = vec![];
    out_buffer.read_to_end(&mut new_buf);

    for b in new_buf {
        print!("'{:b}'", b);
    }

    Ok(())
}
