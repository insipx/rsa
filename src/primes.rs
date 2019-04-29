mod gen;
mod primedb;
const MINIMUM_KEY_LENGTH: usize = 512;

pub enum KeySize {
    FiveTwelve,
    TenTwentyFour,
    TwentyFourtyEight,
    FourtyNinetySix,
    EightyOneNinetyTwo,
}

impl KeySize {
    fn as_num(&self) -> usize {
        match *self {
            KeySize::FiveTwelve => 512,
            KeySize::TenTwentyFour => 1024,
            KeySize::TwentyFourtyEight => 2048,
            KeySize::FourtyNinetySix => 4096,
            KeySize::EightyOneNinetyTwo => 8192,
        }
    }
}

impl Default for KeySize {
    fn default() -> Self {
        KeySize::TwentyFourtyEight
    }
}
