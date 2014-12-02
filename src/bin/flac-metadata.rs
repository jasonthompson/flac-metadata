extern crate flac_metadata;

use std::io::{File, BufferedReader};
use std::os;

use flac_metadata::block;

struct Parser<'a> {
    reader: &'a mut (Reader + 'a),
}

impl<'a, R: Reader> Parser<'a> {
    fn new<'a>(reader: &'a mut BufferedReader<R>) -> Parser<'a> {
        Parser {
            reader: reader
        }
    }

     fn next_header(&mut self) -> Result<block::BlockHeader, &'static str> {
         let header_bytes = self.reader.read_exact(4).unwrap();
         let header: block::BlockHeader = try!(block::BlockHeader::parse(&header_bytes));
         Ok(header)
    }
}

#[allow(dead_code)]
fn is_flac(first_four_bytes: &Vec<u8>) -> bool {
    *first_four_bytes == vec![102, 76, 97, 67]   // "fLaC"
}

#[allow(dead_code)]   
fn main() {
    let args = os::args();
    let file_name = &args[1];
    
    let path = Path::new(file_name);
    let mut reader: &mut BufferedReader<File> = &mut BufferedReader::new(File::open(&path).unwrap());
    let mut parser = Parser::new(reader);
    
    let first_bytes = parser.reader.read_exact(4).unwrap();

    if is_flac(&first_bytes) {
        println!("{}", parser.next_header().unwrap());
        // let stream_info_bytes: Vec<u8> = parser.reader.read_exact(header.block_length).unwrap();
        // let stream_info = block::StreamInfoBlock::parse(&stream_info_bytes);
        // println!("{}", stream_info_bytes);
        // println!("{}", stream_info.unwrap());
    }
}        
