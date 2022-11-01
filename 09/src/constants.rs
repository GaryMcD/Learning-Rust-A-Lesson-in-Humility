use crate::Word;
use crate::Hash;

pub const INITIAL_HASH: Hash = Hash {
	a: Word(0x6a09e667),
	b: Word(0xbb67ae85), 
	c: Word(0x3c6ef372), 
	d: Word(0xa54ff53a),
	e: Word(0x510e527f),
   f: Word(0x9b05688c), 
   g: Word(0x1f83d9ab),
   h: Word(0x5be0cd19)
};

pub const COMPUTATION_CONSTANTS: [Word; 64] =
[
    Word(0x428a2f98), Word(0x71374491), 
    Word(0xb5c0fbcf), Word(0xe9b5dba5),
    Word(0x3956c25b), Word(0x59f111f1), 
    Word(0x923f82a4), Word(0xab1c5ed5),
    Word(0xd807aa98), Word(0x12835b01), 
    Word(0x243185be), Word(0x550c7dc3),
    Word(0x72be5d74), Word(0x80deb1fe), 
    Word(0x9bdc06a7), Word(0xc19bf174),
    Word(0xe49b69c1), Word(0xefbe4786), 
    Word(0x0fc19dc6), Word(0x240ca1cc),
    Word(0x2de92c6f), Word(0x4a7484aa), 
    Word(0x5cb0a9dc), Word(0x76f988da),
    Word(0x983e5152), Word(0xa831c66d), 
    Word(0xb00327c8), Word(0xbf597fc7),
    Word(0xc6e00bf3), Word(0xd5a79147), 
    Word(0x06ca6351), Word(0x14292967),
    Word(0x27b70a85), Word(0x2e1b2138), 
    Word(0x4d2c6dfc), Word(0x53380d13),
    Word(0x650a7354), Word(0x766a0abb), 
    Word(0x81c2c92e), Word(0x92722c85),
    Word(0xa2bfe8a1), Word(0xa81a664b), 
    Word(0xc24b8b70), Word(0xc76c51a3),
    Word(0xd192e819), Word(0xd6990624), 
    Word(0xf40e3585), Word(0x106aa070),
    Word(0x19a4c116), Word(0x1e376c08), 
    Word(0x2748774c), Word(0x34b0bcb5),
    Word(0x391c0cb3), Word(0x4ed8aa4a), 
    Word(0x5b9cca4f), Word(0x682e6ff3),
    Word(0x748f82ee), Word(0x78a5636f), 
    Word(0x84c87814), Word(0x8cc70208),
    Word(0x90befffa), Word(0xa4506ceb), 
    Word(0xbef9a3f7), Word(0xc67178f2)
];

