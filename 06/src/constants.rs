use crate::word::Word;

pub const INITIAL_HASH_VALUES: [Word; 8] =
[
	Word { value: 0x6a09e667 }, Word { value: 0xbb67ae85 }, 
	Word { value: 0x3c6ef372 }, Word { value: 0xa54ff53a },
    Word { value: 0x510e527f }, Word { value: 0x9b05688c }, 
    Word { value: 0x1f83d9ab }, Word { value: 0x5be0cd19 }
];

pub const COMPUTATION_CONSTANTS: [Word; 64] =
[
    Word { value: 0x428a2f98 }, Word { value: 0x71374491 }, 
    Word { value: 0xb5c0fbcf }, Word { value: 0xe9b5dba5 },
    Word { value: 0x3956c25b }, Word { value: 0x59f111f1 }, 
    Word { value: 0x923f82a4 }, Word { value: 0xab1c5ed5 },
    Word { value: 0xd807aa98 }, Word { value: 0x12835b01 }, 
    Word { value: 0x243185be }, Word { value: 0x550c7dc3 },
    Word { value: 0x72be5d74 }, Word { value: 0x80deb1fe }, 
    Word { value: 0x9bdc06a7 }, Word { value: 0xc19bf174 },
    Word { value: 0xe49b69c1 }, Word { value: 0xefbe4786 }, 
    Word { value: 0x0fc19dc6 }, Word { value: 0x240ca1cc },
    Word { value: 0x2de92c6f }, Word { value: 0x4a7484aa }, 
    Word { value: 0x5cb0a9dc }, Word { value: 0x76f988da },
    Word { value: 0x983e5152 }, Word { value: 0xa831c66d }, 
    Word { value: 0xb00327c8 }, Word { value: 0xbf597fc7 },
    Word { value: 0xc6e00bf3 }, Word { value: 0xd5a79147 }, 
    Word { value: 0x06ca6351 }, Word { value: 0x14292967 },
    Word { value: 0x27b70a85 }, Word { value: 0x2e1b2138 }, 
    Word { value: 0x4d2c6dfc }, Word { value: 0x53380d13 },
    Word { value: 0x650a7354 }, Word { value: 0x766a0abb }, 
    Word { value: 0x81c2c92e }, Word { value: 0x92722c85 },
    Word { value: 0xa2bfe8a1 }, Word { value: 0xa81a664b }, 
    Word { value: 0xc24b8b70 }, Word { value: 0xc76c51a3 },
    Word { value: 0xd192e819 }, Word { value: 0xd6990624 }, 
    Word { value: 0xf40e3585 }, Word { value: 0x106aa070 },
    Word { value: 0x19a4c116 }, Word { value: 0x1e376c08 }, 
    Word { value: 0x2748774c }, Word { value: 0x34b0bcb5 },
    Word { value: 0x391c0cb3 }, Word { value: 0x4ed8aa4a }, 
    Word { value: 0x5b9cca4f }, Word { value: 0x682e6ff3 },
    Word { value: 0x748f82ee }, Word { value: 0x78a5636f }, 
    Word { value: 0x84c87814 }, Word { value: 0x8cc70208 },
    Word { value: 0x90befffa }, Word { value: 0xa4506ceb }, 
    Word { value: 0xbef9a3f7 }, Word { value: 0xc67178f2 }
];
