use std::io::{File, BufferedReader};
use std::os;
use std::str;
use std::fmt::{mod, Show, Formatter};

fn is_flac(first_four_bytes: &[u8]) -> bool {
    first_four_bytes == [102, 76, 97, 67]   // "fLaC"
}

#[deriving(Show, Eq, PartialEq)]
pub enum BlockType {
    StreamInfo,
    Padding,
    Application,
    SeekTable,
    VorbisComment,
    Cuesheet,
    Picture,
}

pub struct BlockHeader {
    is_last_block: bool,
    block_type: BlockType,
    block_length: uint,
}

impl BlockHeader {
    fn parse(header_bytes: &[u8]) -> Result<BlockHeader, &'static str> {
        let first_bit = header_bytes[0] & 0x1;
        let is_last_block = first_bit == 1;
        let block_type_bits = header_bytes[0] << 1;

        let block_type = BlockHeader::get_block_type(block_type_bits as uint).unwrap();
        let block_length = bits_to_uint_24(header_bytes.slice(1,4));
        
        Ok(BlockHeader {
            is_last_block: is_last_block,
            block_type: block_type,
            block_length: block_length,
        })
    }

    fn get_block_type(block_type_bits: uint) -> Result<BlockType, &'static str> {
        let block_type_dict = [StreamInfo, Padding, Application, SeekTable, 
                               VorbisComment, Cuesheet, Picture];        
        Ok(block_type_dict[block_type_bits as uint])
    }
    
}

fn bits_to_uint_16(number_bytes: &[u8]) -> uint {
    (number_bytes[1] as u16 + (number_bytes[0] as u16 << 8)) as uint
}

fn bits_to_uint_24(number_bytes: &[u8]) -> uint {
    (number_bytes[2] as u32 + 
     (number_bytes[1] as u32 << 8) + 
     (number_bytes[0] as u32 << 16)) as uint
}

impl Show for BlockHeader {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        try!(write!(f, "
BLOCK HEADER:
    Block type: {} 
    Last block: {} 
    Block length: {}", self.is_last_block, self.block_type, self.block_length));
        Ok(())
    }
}

pub struct StreamInfoBlock {
    minimum_block_size: uint,
    maximum_block_size: uint,
    minimum_frame_size: uint,
    maximum_frame_size: uint,
    sample_rate: uint,
    number_of_channels: uint,
    bits_per_sample: uint,
    total_samples: uint,
    audio_data_md5_signature: &'static str,
}

impl StreamInfoBlock {
    fn parse(block_bytes: &[u8]) -> Result<StreamInfoBlock, &'static str> {
        let total_samples = ((block_bytes[13] as u64 << 4) +
                             (block_bytes[14] as u64 << 12) +
                             (block_bytes[15] as u64 << 28) +
                             (block_bytes[16] as u64 << 60) +
                             (block_bytes[17] as u64 >> 4)) as uint;
        Ok(StreamInfoBlock {
            minimum_block_size: bits_to_uint_16(block_bytes.slice(0,2)),
            maximum_block_size: bits_to_uint_16(block_bytes.slice(2,4)), 
            minimum_frame_size: bits_to_uint_24(block_bytes.slice(4,7)),
            maximum_frame_size: bits_to_uint_24(block_bytes.slice(7,10)),
            // 20 bits
            sample_rate: bits_to_uint_24(block_bytes.slice(10,13)) >> 4,
            // 3 bits 
            number_of_channels: (block_bytes[12] >> 5) as uint,
            // 5 bits
            bits_per_sample: (block_bytes[12] << 3) as uint,
            // 36 bits
            total_samples: total_samples,
            audio_data_md5_signature: fixme,
        })
    }
}

impl Show for StreamInfoBlock {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        try!(write!(f, "
STREAMINFO BLOCK:
    Minimum block size: {}
    Maximum block size: {}
    Minimum frame size: {}
    Maximum frame size: {}
    Sample rate: {}
    Number of channels: {}
    Bits per sample: {}
    Total samples: {}
    Audio data MD5 signature: {}
", self.minimum_block_size, self.maximum_block_size, self.minimum_frame_size, self.maximum_frame_size, self.sample_rate, self.number_of_channels, self.bits_per_sample, self.total_samples, self.audio_data_md5_signature));
        Ok(())
    }
}
    
        
fn main() {
    let args = os::args();
    let file_name = &args[1];
    
    let path = Path::new(file_name);
    let mut reader = BufferedReader::new(File::open(&path));
    let first_bytes = reader.read_exact(4).unwrap();

    if is_flac(first_bytes.as_slice()) {
        let header_bytes = reader.read_exact(4).unwrap();
        let header = BlockHeader::parse(header_bytes.as_slice()).unwrap();
        println!("{}", header);
        let stream_info_bytes = reader.read_exact(header.block_length).unwrap();
        let stream_info = StreamInfoBlock::parse(stream_info_bytes.as_slice());
        println!("{}", stream_info);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_block_length() {
        let block_bits = [0x00, 0x00, 0x00, 0x14];
        let header = super::BlockHeader::parse(block_bits).unwrap();
        assert_eq!(10066176, header.block_length);
        assert_eq!(false, header.is_last_block);
        assert_eq!(super::StreamInfo, header.block_type);
    }
}
        
