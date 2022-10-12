use crate::word::Word;

#[derive(Debug)]
pub struct Message {
    pub raw_input: Vec<u8>,
}

#[derive(Debug)]
pub struct MessageBlock {
    pub words: Vec<Word>,
}

#[derive(Debug)]
pub struct MessageSchedule {
	pub words: [Word; 64],
}

impl Message {
    pub fn into_message_blocks(self) -> Vec<MessageBlock> {
        let (mut words, mut leftovers) = self.into_words_with_leftovers();

        // Pad with single 1 bit (which is 128 in u8)
        {
            leftovers.push(128);
            while leftovers.len() < 4 {
                leftovers.push(0);
            }
            words.push( Word { value: u32::from_be_bytes(leftovers.try_into().unwrap()),})
        }

        // Pad With Zeros
        {
            let number_of_padding_words = 
            16 - (
                (   (words.len() as u64) // current length
                    + 2 ) // length with 64bit length on end
                % 16); // gap between length and next message block length
                
            for _ in 0..number_of_padding_words {
                words.push(Word { value: 0,});
            }
        }

        // Add 64 bit length on end
        {
            let length = self.raw_input.len() as u64 * 8;

            length
                .to_be_bytes()
                .chunks(4)
                .map(|chunk| Word { value: u32::from_be_bytes(chunk.try_into().unwrap()),})
                .for_each(|word| words.push(word));
        }

        words.chunks(16)
            .map(|words_mb| MessageBlock { words: words_mb.to_vec(),})
            .collect()
    }

    fn into_words_with_leftovers(&self) -> (Vec<Word>,Vec<u8>) {
        let mut words: Vec<Word> = Vec::new();
        let mut leftovers: Vec<u8> = Vec::new();

        self.raw_input
            .chunks(4)
            .for_each(|chunk| {
                if chunk.len() == 4 {
                    words.push( Word { value: u32::from_be_bytes(chunk.try_into().unwrap()),});
                } else {
                    leftovers = chunk.to_vec();
                }
            });
        
        (words, leftovers)
    }
}

impl MessageBlock {
	pub fn into_message_schedule(&self) -> MessageSchedule {
		let mut schedule: [Word; 64] = [Word { value: 0 }; 64];
		
		for index in 0..16 {
			schedule[index] = self.words[index];
		}

		for index in 16..64 {
			let w2 = schedule[index-2];
			let w7 = schedule[index-7];
			let w15 = schedule[index-15];
			let w16 = schedule[index-16];

			schedule[index] = w2.σ_major() + w7 + w15.σ_minor() + w16;
		}

		MessageSchedule { words: schedule }
	}
}
