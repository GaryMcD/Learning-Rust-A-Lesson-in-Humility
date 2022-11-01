use super::MessageBlock;
use crate::Word;

pub struct RawInputHandler( pub Vec<u8> );

impl RawInputHandler {
	pub fn into_message_blocks( &self ) -> Vec<MessageBlock> {
		let length = self.0.len() as u64 * 8;
		let ( mut words, mut leftovers ) = self.into_words_with_leftovers();
		
		// Pad with single 1 bit (which is 128 in u8)
		{
		    leftovers.push( 128 );
		    while leftovers.len() < 4 {
		        leftovers.push( 0 );
		    }
		    words.push( Word( u32::from_be_bytes( leftovers.try_into().unwrap() ) ) )
		}

		// Pad With Zeros
		{
		    let number_of_padding_words = 
		    16 - (
		        (   ( words.len() as u64 ) // current length
		            + 2 ) // length with 64bit length on end
		        % 16 ); // gap between length and next message block length
		        
		    for _ in 0..number_of_padding_words {
		        words.push( Word( 0 ) );
		    }
		}

		// Add 64 bit length on end
		{
		     length
		        .to_be_bytes()
		        .chunks( 4 )
		        .map( |chunk| Word( u32::from_be_bytes( chunk.try_into().unwrap() ) ) )
		        .for_each( |word| words.push( word ) );
		}

		words.chunks( 16 )
		    .map( |words_mb| MessageBlock( words_mb.to_vec() ) )
		    .collect()
	}


	fn into_words_with_leftovers( &self ) -> ( Vec<Word>, Vec<u8> ) {
	    let mut words: Vec<Word> = Vec::new();
	    let mut leftovers: Vec<u8> = Vec::new();

	    self.0
	        .chunks(4)
	        .for_each( |chunk| {
	            if chunk.len() == 4 {
	                words.push( Word( u32::from_be_bytes( chunk.try_into().unwrap() ) ) );
	            } else {
	                leftovers = chunk.to_vec();
	            }
	        });
	    
	    ( words, leftovers )
	}
}

