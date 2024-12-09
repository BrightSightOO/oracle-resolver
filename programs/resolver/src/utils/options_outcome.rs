#[derive(Clone, Copy, PartialEq, Eq)]
pub enum OptionsOutcome {
    Outcome { outcome: u8 },
    Invalid,
}

impl OptionsOutcome {
    pub const fn from_value(value: u64, options: u8) -> OptionsOutcome {
        if value < 2 || value > 2 + options as u64 {
            OptionsOutcome::Invalid
        } else {
            OptionsOutcome::Outcome { outcome: (value - 2) as u8 }
        }
    }
}

impl From<OptionsOutcome> for cpi::hpl::parimutuel::ResolveV1Args {
    #[inline]
    fn from(value: OptionsOutcome) -> Self {
        match value {
            OptionsOutcome::Outcome { outcome } => Self::Outcome { outcome },
            OptionsOutcome::Invalid => Self::Invalid,
        }
    }
}

impl From<OptionsOutcome> for cpi::hpl::parimutuel_lulo::ResolveV1Args {
    #[inline]
    fn from(value: OptionsOutcome) -> Self {
        match value {
            OptionsOutcome::Outcome { outcome } => Self::Outcome { outcome },
            OptionsOutcome::Invalid => Self::Invalid,
        }
    }
}
