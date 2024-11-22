mod asserts;

pub use self::asserts::*;

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum YesNo {
    Yes = 1,
    No = 2,
    Invalid = 3,
}

impl YesNo {
    pub const fn from_value(value: u64) -> YesNo {
        match value {
            1 => YesNo::Yes,
            2 => YesNo::No,
            _ => YesNo::Invalid,
        }
    }
}

impl From<YesNo> for cpi::hpl::p2p::Outcome {
    #[inline]
    fn from(value: YesNo) -> Self {
        match value {
            YesNo::Yes => Self::Yes,
            YesNo::No => Self::No,
            YesNo::Invalid => Self::Invalid,
        }
    }
}
