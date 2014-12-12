extern crate flac_metadata;

use std::io::{File, BufferedReader};
use std::os;

use flac_metadata::block;

struct Parser<'a> {
    reader: &'a mut (Reader + 'a),
    done: bool,
}

impl<'a> Parser<'a> {
    fn new<'a, R: Reader>(reader: &'a mut BufferedReader<R>) -> Result<Parser<'a>, &'static str> {
        let first_bytes = reader.read_exact(4).unwrap();
        if first_bytes == vec![102, 76, 97, 67] {      // "fLaC"
            Ok(Parser {
                reader: reader,
                done: false,
            })
        } else {
            Err("This is not a FLAC file")
        }
    }
    
    fn next_block_header<'a>(&'a mut self) -> block::BlockHeader {
        let header_bytes = self.reader.read_exact(4).unwrap();
        let block_header = block::BlockHeader::new(&header_bytes);

        self.done = block_header.is_last_block;
        block_header
    }

    fn next_block<'a>(&'a mut self) {
        let next_header = self.next_block_header();

  
        let block_length = next_header.block_length;
        let block_type = next_header.block_type;
        let block_bytes = self.reader.read_exact(block_length).unwrap();

        // Using println! here, but for the library I'd be returning the block itself or
        // adding it to a stack of blocks.
        match block_type {
            block::BlockType::StreamInfo => println!("{}", block::Block::new(&block_bytes, next_header)),
            block::BlockType::Padding => println!("{}", next_header),
            block::BlockType::Application => println!("{}", next_header),
            block::BlockType::SeekTable => println!("{}", next_header),
            block::BlockType::VorbisComment => println!("{}", next_header),
            block::BlockType::Cuesheet => println!("{}", next_header),
            block::BlockType::Picture => println!("{}", next_header),
        }
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
        while parser.done != true {
            parser.next_block();
        }
    }    
}        
 
