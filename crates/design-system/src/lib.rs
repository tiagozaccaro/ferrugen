#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorScheme { Light, Dark, System }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Density { Compact, Comfortable }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DesignPreferences {
    pub color_scheme: ColorScheme,
    pub density: Density,
}

impl Default for DesignPreferences {
    fn default() -> Self {
        Self { color_scheme: ColorScheme::System, density: Density::Comfortable }
    }
}
