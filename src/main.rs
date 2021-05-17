mod encoding;
mod ioutils;
mod priorityq;

use encoding::EncodingNode;
use ioutils::{read_header, write_header, HuffmanPathWriter};
use priorityq::PriorityQ;

use std::collections::BTreeMap;
use std::io::Read;

use crate::encoding::Direction;

fn main() -> anyhow::Result<()> {
    let input_file = std::fs::File::open("src/main.rs").expect("could not open file");
    let bytes = input_file.bytes();
    let in_buffer = "aaaabbbccd".bytes();

    // Make byte histogram (this could simply be an array too)
    let mut hist: BTreeMap<u8, usize> = BTreeMap::new();
    for byte in in_buffer.clone() {
        print!("{:b}", byte);
        let i = byte;
        *hist.entry(i).or_insert(0) += 1;
    }

    println!();

    // println!("len: {}", hist.len());

    // map histogram to huffman tree nodes, and build priority que
    let mut iter = hist.iter();
    let (k, v) = iter.next().expect("at leat one element");
    let mut queue = PriorityQ::new(EncodingNode::new_leaf(*k, *v));

    for (k, v) in iter {
        let node = EncodingNode::new_leaf(*k, *v);
        queue = queue.enque(node);
    }

    // println!("q len: {}", queue.len());

    // Loop throught priority que, combining nodes to form tree
    let root = queue.reduce();

    // println!("{:?}", root.count_leaves());
    // println!("height: {}", root.height());

    // println!("{:#?}", root);

    let st = root.to_symbol_table();

    // println!("st len: {}", st.len());
    let mut out_buffer: Vec<u8> = vec![];
    // write_header(&mut out_buffer, &hist)?;

    let mut path_writer = HuffmanPathWriter::new();

    for b in in_buffer {
        match st.get(&b) {
            Some(path) => {
                for p in path {
                    match p {
                        Direction::Left => print!("{}", 1),
                        Direction::Right => print!("{}", 0),
                    };
                }
                path_writer.write_path(&mut out_buffer, path)?;
            }
            None => anyhow::bail!("key should never be empty"),
        }
    }

    println!();

    for b in out_buffer {
        print!("{:b}", b);
    }

    println!();
    // for (k, v) in st.iter() {
    // println!("({}, {:?})", k, v);
    // }

    // let r = queue.pop();
    // println!("{:?}", &r);

    /*
    TODO:
    3. pop each node off the priority que building up a single node,
    4. encode data
    */

    Ok(())
}
