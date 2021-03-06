
/// Implemented by contexts that can add square border.
pub trait AddSquareBorder<'a, T> {
    /// Adds a square border radius.
    fn square_border_radius(&'a self, radius: f64) -> T;

    /// Adds a square border width.
    #[inline(always)]
    fn square_border_width(&'a self, width: f64) -> T {
        self.square_border_radius(0.5 * width)
    }
}

