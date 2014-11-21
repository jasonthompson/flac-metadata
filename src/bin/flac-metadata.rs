extern crate flac_metadata;

use std::io::{File, BufferedReader};
use std::os;

use flac_metadata::block;

#[allow(dead_code)]
fn is_flac(first_four_bytes: &[u8]) -> bool {
    first_four_bytes == [102, 76, 97, 67]   // "fLaC"
}

#[allow(dead_code)]   
fn main() {
    let args = os::args();
    let file_name = &args[1];
    
    let path = Path::new(file_name);
    let mut reader = BufferedReader::new(File::open(&path));
    let first_bytes = reader.read_exact(4).unwrap();

    if is_flac(first_bytes.as_slice()) {
        let header_bytes = reader.read_exact(4).unwrap();
        let header = block::BlockHeader::parse(header_bytes.as_slice()).unwrap();
        println!("{}", header);
        let stream_info_bytes = reader.read_exact(header.block_length).unwrap();
        let stream_info = block::StreamInfoBlock::parse(stream_info_bytes.as_slice());
        println!("{}", stream_info_bytes);
        println!("{}", stream_info.unwrap());
    }
}        
