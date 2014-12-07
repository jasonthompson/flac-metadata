extern crate flac_metadata;

use std::io::{File, BufferedReader};
use std::os;

use flac_metadata::block;

struct Parser<'a> {
    reader: &'a mut (Reader + 'a),
}

impl<'a> Parser<'a> {
    fn new<'a, R: Reader>(reader: &'a mut BufferedReader<R>) -> Result<Parser<'a>, &'static str> {
        let first_bytes = reader.read_exact(4).unwrap();
        if first_bytes == vec![102, 76, 97, 67] {      // "fLaC"
            Ok(Parser {
                reader: reader
            })
        } else {
            Err("This is not a FLAC file")
        }
    }
    
    fn next_block_header<'a>(&'a mut self) -> block::BlockHeader {
        let header_bytes = self.reader.read_exact(4).unwrap();
        block::BlockHeader::new(&header_bytes)
    }

    fn parse<'a>(&'a mut self) {
        let next_header = self.next_block_header();
        
        let block_bytes = self.reader.read_exact(next_header.block_length).unwrap();
        let block: block::StreamInfoBlock = block::Block::new(&block_bytes, next_header);

        println!("{}", block);
    }
}

const USAGE: &'static str = "
FLAC Audio File Metadata Reader

Usage:
    flac-metadata [OPTIONS]... FILE

Options:
";
    
#[allow(dead_code)]   
fn main() {
    let args = os::args();

    if args.len() == 1 {
        println!("Please provide the name of a FLAC audio file.\n {}", USAGE);
    } else {
        let path = Path::new(&args[1]);
        let mut reader = &mut BufferedReader::new(File::open(&path).unwrap());
        let mut parser = &mut Parser::new(reader).unwrap();
        parser.parse();
    }    
}        
 
