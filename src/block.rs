use std::fmt::{mod, Show, Formatter};
use serialize::hex::ToHex;

use util;


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
    pub block_length: uint,
}

impl BlockHeader {
    pub fn parse(header_bytes: &[u8]) -> Result<BlockHeader, &'static str> {
        let first_bit = header_bytes[0] & 0x1;
        let is_last_block = first_bit == 1;
        let block_type_bits = header_bytes[0] << 1;

        let block_type = BlockHeader::get_block_type(block_type_bits as uint).unwrap();

        let block_length = util::bits_to_uint_24(header_bytes.slice(1,4));
        
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


impl Show for BlockHeader {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        try!(write!(f, "
BLOCK HEADER:
    Block type: {} 
    Last block: {} 
    Block length: {}", self.block_type, self.is_last_block, self.block_length));
        Ok(())
    }
}

pub struct StreamInfoBlock<'a> {
    minimum_block_size: uint,
    maximum_block_size: uint,
    minimum_frame_size: uint,
    maximum_frame_size: uint,
    sample_rate: uint,
    number_of_channels: uint,
    bits_per_sample: uint,
    total_samples: uint, 
    audio_data_md5_signature: &'a [u8],
}

impl<'a> StreamInfoBlock<'a> {
    pub fn parse(block_bytes: &[u8]) -> Result<StreamInfoBlock, &'static str> {
        let total_samples = ((block_bytes[13] as u64 << 60) +
                             (block_bytes[14] as u64 << 32) +
                             (block_bytes[15] as u64 << 16) +
                             (block_bytes[16] as u64 << 8) +
                             (block_bytes[17] as u64)) as uint;
        let md5 = block_bytes.slice(18,34);
        
        Ok(StreamInfoBlock {
            minimum_block_size: util::bits_to_uint_16(block_bytes.slice(0,2)),
            maximum_block_size: util::bits_to_uint_16(block_bytes.slice(2,4)), 
            minimum_frame_size: util::bits_to_uint_24(block_bytes.slice(4,7)),
            maximum_frame_size: util::bits_to_uint_24(block_bytes.slice(7,10)),
            // 20 bits
            sample_rate: util::bits_to_uint_24(block_bytes.slice(10,13)) >> 4,
            // 3 bits 
            number_of_channels: (block_bytes[12] >> 5) as uint,
            // 5 bits
            bits_per_sample: (block_bytes[12] << 3) as uint,
            // 36 bits
            total_samples: total_samples,
            audio_data_md5_signature: md5,
        })
    }
}
    
impl<'a> Show for StreamInfoBlock<'a> {
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
    Audio data MD5 signature: {:}
", self.minimum_block_size, self.maximum_block_size, self.minimum_frame_size, self.maximum_frame_size, self.sample_rate, self.number_of_channels, self.bits_per_sample, self.total_samples, self.audio_data_md5_signature.to_hex()));
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use serialize::hex::ToHex;

    #[test]
    fn test_block_length() {
        let block_bits = [0x00, 0x00, 0x00, 0x14];
        let header = super::BlockHeader::parse(block_bits).unwrap();
        assert_eq!(20, header.block_length);
        assert_eq!(false, header.is_last_block);
        assert_eq!(super::StreamInfo, header.block_type);
    }
    #[test]
    fn test_stream_info_parse() {
        let block_bytes = [16, 0, 16, 0, 0, 0, 16, 0, 55, 204, 10, 196, 66, 240, 0, 161, 235, 180, 134, 228, 11, 72,
                           80, 182, 87, 11, 41, 90, 91, 38, 134, 143, 114, 67];
        let block = super::StreamInfoBlock::parse(block_bytes).unwrap();
        assert_eq!(block.minimum_block_size, 4096u);
        assert_eq!(block.maximum_block_size, 4096u);
        assert_eq!(block.minimum_frame_size, 16u);
        assert_eq!(block.maximum_frame_size, 14284u);
        assert_eq!(block.sample_rate, 44100u);
        assert_eq!(block.bits_per_sample, 16u);
        assert_eq!(block.number_of_channels, 2u);
        assert_eq!(block.total_samples, 10611636u);
        assert_eq!(block.audio_data_md5_signature.to_hex(), "86e40b4850b6570b295a5b26868f7243".to_string());
        }
}