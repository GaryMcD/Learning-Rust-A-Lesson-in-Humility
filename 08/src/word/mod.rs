#[derive(Debug, Clone, Copy)]
pub struct Word(pub u32);

impl Word {
	pub fn σ_minor(self) -> Word {
		let rotr_7 = self.rotr(7);
		let rotr_18 = self.rotr(18);
		let shr_3 = self >> 3;
		rotr_7 ^ rotr_18 ^ shr_3
	}
	pub fn σ_major(self) -> Word {
		let rotr_17 = self.rotr(17);
		let rotr_19 = self.rotr(19);
		let shr_10 = self >> 10;
		rotr_17 ^ rotr_19 ^ shr_10
	}
	pub fn sigma_uc_minor(self) -> Word {
		let rotr_2 = self.rotr(2);
		let rotr_13 = self.rotr(13);
		let rotr_22 = self.rotr(22);
		rotr_2 ^ rotr_13 ^ rotr_22
	}
	pub fn sigma_uc_major(self) -> Word {
		let rotr_6 = self.rotr(6);
		let rotr_11 = self.rotr(11);
		let rotr_25 = self.rotr(25);
		rotr_6 ^ rotr_11 ^ rotr_25
	}
	pub fn choice(x: Word, y: Word, z: Word) -> Word {
		(x & y) ^ ((!x) & z)
	}
	pub fn majority(x: Word, y: Word, z: Word) -> Word {
		(x & y) ^ (x & z) ^ (y & z)
	}
	pub fn rotr(self, rotation_amount: u32) -> Word {
		let lhs = self >> rotation_amount;
		let rhs = self << (32 - rotation_amount);
		lhs | rhs
	}
	pub fn temporary_word_one(e: Word, f: Word, g: Word, h: Word, K: Word, W: Word) -> Word {
		h + Word::sigma_uc_major(e) + Word::choice(e,f,g) + K + W
	}
	pub fn temporary_word_two(a: Word, b: Word, c: Word) -> Word {
		Word::sigma_uc_minor(a) + Word::majority(a,b,c)
	}
}

impl std::ops::Add<Word> for Word {
	type Output = Word;
	fn add(self, rhs: Word) -> Word {
		Word(self.0.wrapping_add(rhs.0))
	}
}

impl std::ops::BitAnd<Word> for Word {
	type Output = Word;
	fn bitand(self, rhs: Word) -> Word {
		Word(self.0 & rhs.0)
	}
}

impl std::ops::BitOr<Word> for Word {
	type Output = Word;
	fn bitor(self, rhs: Word) -> Word {
		Word(self.0 | rhs.0)
	}
}

impl std::ops::BitXor<Word> for Word {
	type Output = Word;
	fn bitxor(self, rhs: Word) -> Word {
		Word(self.0 ^ rhs.0)
	}
}

impl std::ops::Not for Word {
	type Output = Word;
	fn not(self) -> Word {
		Word(!self.0)
	}
}

impl std::ops::Shr<u32> for Word {
	type Output = Word;
	fn shr(self, rhs: u32) -> Word {
		Word(self.0 >> rhs)
	}
}

impl std::ops::Shl<u32> for Word {
	type Output = Word;
	fn shl(self, rhs: u32) -> Word {
		Word(self.0 << rhs)
	}
}

impl PartialEq for Word {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl Eq for Word {}
