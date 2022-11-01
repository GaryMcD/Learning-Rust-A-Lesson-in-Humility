use crate::constants::COMPUTATION_CONSTANTS;

use crate::MessageSchedule;

use crate::Word;

#[derive(Debug)]
#[derive(Clone)]
#[derive(Copy)]
pub struct Hash {
    pub a: Word,
    pub b: Word,
    pub c: Word,
    pub d: Word,
    pub e: Word,
    pub f: Word,
    pub g: Word,
    pub h: Word,
}

impl Hash {
	pub fn compute(initial_hash: Hash, message_schedule: MessageSchedule) -> Hash {
		let mut working_hash = initial_hash.clone();
		
		for index in 0..64 {
			let t1 = Word::temporary_word_one(
				working_hash.e,
				working_hash.f,
				working_hash.g,
				working_hash.h,
				COMPUTATION_CONSTANTS[index],
				message_schedule.0[index]
			);
			let t2 = Word::temporary_word_two(
				working_hash.a,
				working_hash.b,
				working_hash.c,
			);

			working_hash = Hash {
				a: t1 + t2,
				b: working_hash.a,
				c: working_hash.b,
				d: working_hash.c,
				e: working_hash.d + t1,
				f: working_hash.e,
				g: working_hash.f,
				h: working_hash.g
			};
		}

		Hash {
			a: initial_hash.a + working_hash.a,
			b: initial_hash.b + working_hash.b,
			c: initial_hash.c + working_hash.c,
			d: initial_hash.d + working_hash.d,
			e: initial_hash.e + working_hash.e,
			f: initial_hash.f + working_hash.f,
			g: initial_hash.g + working_hash.g,
			h: initial_hash.h + working_hash.h,
		}
	}
}
