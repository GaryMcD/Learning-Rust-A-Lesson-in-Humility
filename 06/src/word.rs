#[derive(Debug)]
#[derive(Clone)]
#[derive(Copy)]
pub struct Word {
   pub value: u32,
}

impl Word {
	pub fn σ_minor(&self) -> Word {
		let rotr_7 = self.rotr(7);
		let rotr_18 = self.rotr(18);
		let shr_3 = self >> 3;
		rotr_7 ^ rotr_18 ^ shr_3
	}
	pub fn σ_major(&self) -> Word {
		let rotr_17 = self.rotr(17);
		let rotr_19 = self.rotr(19);
		let shr_10 = self >> 10;
		rotr_17 ^ rotr_19 ^ shr_10
	}
	pub fn rotr(&self, rotation_amount: u32) -> Word {
		let lhs = self >> rotation_amount;
		let rhs = self << (32 - rotation_amount);
		lhs | rhs
	}
}

impl std::ops::Add<Word> for Word {
	type Output = Word;
	fn add(self, rhs: Word) -> Word {
		Word { value: self.value.wrapping_add(rhs.value) }
	}
}

impl std::ops::BitOr<Word> for Word {
	type Output = Word;
	fn bitor(self, rhs: Word) -> Word {
		Word { value: self.value | rhs.value }
	}
}

impl std::ops::BitXor<Word> for Word {
	type Output = Word;
	fn bitxor(self, rhs: Word) -> Word {
		Word { value: self.value ^ rhs.value }
	}
}

impl std::ops::Shr<u32> for &Word {
	type Output = Word;
	fn shr(self, rhs: u32) -> Word {
		Word { value: self.value >> rhs }
	}
}

impl std::ops::Shl<u32> for &Word {
	type Output = Word;
	fn shl(self, rhs: u32) -> Word {
		Word { value: self.value << rhs }
	}
}
