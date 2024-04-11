#[derive(Debug, Clone)]
pub struct Palette {
    fg: [f32; 4],
    bg: [f32; 4],
}

impl Palette {
    const PALETTES: [(u32, u32); 6] = [
        // #EA2F20, #130825
        (0xEA2F20, 0x130825),
        // #93C2AB, #9474AD
        (0x93C2AB, 0x09474D),
        // #09474D, #93C2AB
        (0x09474D, 0x93C2AB),
        // #6974CC, #0D040F
        (0x6974CC, 0x0D040F),
        // #90A3AD, #0F2B15
        (0x90A3AD, 0x0F2B15),
        // #8A8092, #3e0416
        (0x8A8092, 0x3E0416),
    ];

    const PALETTE_LENGTH: usize = Self::PALETTES.len();

    /// Returns the selected palette with the given alpha component.
    ///
    /// Index must be: 0 <= index < 6
    ///
    /// # Errors
    ///
    /// If 0 <= index < 6 is not true, this function will panic
    pub fn pick(index: usize, f_alpha: f32, b_alpha: f32) -> Palette {
        debug_assert!((0..Self::PALETTE_LENGTH).contains(&index));
        let (fg, bg) = Self::PALETTES[index];
        let (fg, bg) = (Self::normalize(fg, f_alpha), Self::normalize(bg, b_alpha));
        Palette { bg, fg }
    }

    /// Returns a random background and foreground color from a predefined
    /// palette clamped between the range 0 to 1
    pub fn random<R: rand::Rng>(rng: &mut R, fg_alpha: f32, bg_alpha: f32) -> Palette {
        let rng = rng.gen_range(0..Self::PALETTE_LENGTH);
        let (fg, bg) = Self::PALETTES[rng];

        let fg = Self::normalize(fg, fg_alpha);
        let bg = Self::normalize(bg, bg_alpha);

        Palette { fg, bg }
    }

    /// Returns the foreground color with no clamping
    pub fn fg(&self) -> &[f32; 4] {
        &self.fg
    }

    /// Returns the background color with no clamping
    pub fn bg(&self) -> &[f32; 4] {
        &self.bg
    }

    #[rustfmt::skip]
    #[inline]
    fn normalize(color: u32, alpha: f32) -> [f32; 4] {
        [
            ((color >> 16) & 0xFF) as f32 / 255.0,
            ((color >>  8) & 0xFF) as f32 / 255.0,
            ((color      ) & 0xFF) as f32 / 255.0,
            alpha,
        ]
    }
}
