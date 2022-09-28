use bitvec::prelude::*;

#[derive(Debug)]
pub struct Word {
   pub bits: BitVec<u8,Msb0>,
}

impl Word {
    pub fn from_u32(input: u32) -> Word {
        let raw_bytes = input.to_be_bytes().to_vec();
        let word_bits = BitVec::<_, Msb0>::from_vec(raw_bytes);

        let word = Word {
            bits: word_bits
        };

        word
    }
}