use bitvec::prelude::*;
use crate::word::Word;

#[derive(Debug)]
pub struct Message {
    pub raw_input: String,
}

impl Message {
    fn bit_length(&self) -> u64 {
        let mut input_length : u64 = 0;

        for _ in self.raw_input.bytes() {
            input_length += 8;
        }

        input_length
    }

    fn number_of_blocks(&self) -> u32 {
        // Size with minimum amount of pre-processing.
        let size_with_preprocessing = self.bit_length() + 1 + 64;

        // + 1 so that values under 512 come back as a single block.
        (size_with_preprocessing as u32 / 512) + 1
    }

    pub fn into_message_blocks(self) -> Vec<MessageBlock> {

        let original_total_length = self.bit_length();
        let original_number_of_blocks = self.number_of_blocks();
        let original_number_of_blocks_usize = usize::try_from(original_number_of_blocks).unwrap();
        
        let byte_vector = self.raw_input.into_bytes();
        let mut raw_bit_vector = BitVec::<_,Msb0>::from_vec(byte_vector);

        let mut blocks: Vec<MessageBlock> = Vec::with_capacity(original_number_of_blocks_usize);

        for block_index in 0..original_number_of_blocks {

            let remaining_bit_vector;
            let mut new_block_bit_vector;

            let is_final_block = block_index == (original_number_of_blocks - 1);
            
            if is_final_block {
                new_block_bit_vector = raw_bit_vector.clone();

                let number_of_zeros = 512 - 64 - 1 - &new_block_bit_vector.len();
                let mut padding_zeros = bitvec![0; number_of_zeros];

                let length_padding = original_total_length.view_bits::<Msb0>();

                new_block_bit_vector.reserve(512 - new_block_bit_vector.capacity()); // Ensure we can hold 512 bits.
                new_block_bit_vector.push(true); // Closing 1
                new_block_bit_vector.append(&mut padding_zeros); // Padding Zeros
                new_block_bit_vector.extend(length_padding); // Encoded length
            } else {
                remaining_bit_vector = raw_bit_vector.split_off(512);
                new_block_bit_vector = raw_bit_vector.clone();
                raw_bit_vector = remaining_bit_vector.clone();
            }

            let new_block = MessageBlock {
                bits: new_block_bit_vector,
            };

            blocks.push(new_block);
        };

        blocks
    }
}

#[derive(Debug)]
pub struct MessageBlock {
    bits: BitVec<u8,Msb0>,
}

impl MessageBlock {
    pub fn into_words(mut self) -> Vec<Word> {

        let mut words: Vec<Word> = Vec::with_capacity(16);

        for word_index in 0..16 {

            let remaining_bits;
            let word_bits;

            if word_index == 15 {
                word_bits = self.bits.clone();
            } else {
                remaining_bits = self.bits.split_off(32);
                word_bits = self.bits.clone();
                self.bits = remaining_bits.clone();
            }

            let new_word = Word {
                bits: word_bits,
            };

            words.push(new_word);
        }

        words
    }
}