use crate::Word;
use super::MessageSchedule;

#[derive(Debug)]
pub struct MessageBlock( pub Vec<Word> );

impl MessageBlock {
	pub fn into_message_schedule( &self ) -> MessageSchedule {
		let mut schedule: [Word; 64] = [Word( 0 ) ; 64];
		
		for index in 0..16 {
			schedule[index] = self.0[index];
		}

		for index in 16..64 {
			let w2 = schedule[index-2];
			let w7 = schedule[index-7];
			let w15 = schedule[index-15];
			let w16 = schedule[index-16];

			schedule[index] = w2.sigma_lc_major() + w7 + w15.sigma_lc_minor() + w16;
		}

		MessageSchedule(schedule)
	}
}
