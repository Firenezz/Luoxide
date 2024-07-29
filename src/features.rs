#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Feature {
    Enabled,
    Disabled,
}

impl Feature {
    #[inline]
    pub fn is_enabled(&self) -> bool {
        matches!(self, Self::Enabled)
    }

    #[inline]
    pub fn is_disabled(&self) -> bool {
        !self.is_enabled()
    }

    #[inline]
    pub fn toggle(&mut self) {
        *self = match self {
            Self::Enabled => Self::Disabled,
            Self::Disabled => Self::Enabled,
        }
    }

    #[inline]
    pub fn enable(&mut self) {
        *self = Self::Enabled
    }

    #[inline]
    pub fn disable(&mut self) {
        *self = Self::Disabled
    }

    #[inline]
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Enabled => "enabled",
            Self::Disabled => "disabled",
        }
    }

    #[inline]
    pub fn as_bool(&self) -> bool {
        self.is_enabled()
    }
}

impl std::fmt::Display for Feature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
//pub configs: Configuration

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ParserFeatures {
    pub labels: Feature,
    pub empty_statements: Feature,
    pub hex_escapes: Feature,
    pub relaxed_breaks: Feature,
}

impl Default for ParserFeatures {
    fn default() -> Self {
        LUA5_3
    }
}

pub(crate) const LUA5_3: ParserFeatures = ParserFeatures {
    labels: Feature::Enabled,
    empty_statements: Feature::Enabled,
    hex_escapes: Feature::Enabled,
    relaxed_breaks: Feature::Enabled,
};
