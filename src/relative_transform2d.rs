
use vecmath::{
    multiply,
    orient,
    rotate_radians,
    scale,
    shear,
    translate,
};
use internal::{
    CanTransform,
    HasTransform,
    Matrix2d,
    Scalar,
};

/// Implemented by contexts that can transform.
pub trait RelativeTransform2d<'a> {
    /// Translate x an y in local coordinates.
    fn trans(&'a self, x: Scalar, y: Scalar) -> Self;
    
    /// Rotates degrees in local coordinates.
    #[inline(always)]
    fn rot_deg(&'a self, angle: Scalar) -> Self {
        let pi: Scalar = Float::pi();
        self.rot_rad(angle * pi / 180.0)
    }

    /// Rotate radians in local coordinates.
    fn rot_rad(&'a self, angle: Scalar) -> Self;

    /// Orients x axis to look at point locally.
    ///
    /// Leaves x axis unchanged if the point to look at is the origin.
    fn orient(&'a self, x: Scalar, y: Scalar) -> Self;

    /// Scales in local coordinates.
    fn scale(&'a self, sx: Scalar, sy: Scalar) -> Self;

    /// Scales in both directions in local coordinates.
    #[inline(always)]
    fn zoom(&'a self, s: Scalar) -> Self {
        self.scale(s, s)
    }

    /// Flips vertically in local coordinates.
    #[inline(always)]
    fn flip_v(&'a self) -> Self {
        self.scale(1.0, -1.0)
    }

    /// Flips horizontally in local coordinates.
    #[inline(always)]
    fn flip_h(&'a self) -> Self {
        self.scale(-1.0, 0.0)
    }

    /// Shears in local coordinates.
    fn shear(&'a self, sx: Scalar, sy: Scalar) -> Self;
}

impl<
    'a,
    T: HasTransform<'a, Matrix2d> + CanTransform<'a, T, Matrix2d>
> RelativeTransform2d<'a> for T {
    #[inline(always)]
    fn trans(&'a self, x: Scalar, y: Scalar) -> T {
        let trans = translate(x, y);
        self.transform(multiply(*self.get_transform(), trans))
    }

    #[inline(always)]
    fn rot_rad(&'a self, angle: Scalar) -> T {
        let rot = rotate_radians(angle);
        self.transform(multiply(*self.get_transform(), rot))
    }

    #[inline(always)]
    fn orient(&'a self, x: Scalar, y: Scalar) -> T {
        let orient = orient(x, y);
        self.transform(multiply(*self.get_transform(), orient))
    }

    #[inline(always)]
    fn scale(&'a self, sx: Scalar, sy: Scalar) -> T {
        let scale = scale(sx, sy);
        self.transform(multiply(*self.get_transform(), scale))
    }

    #[inline(always)]
    fn shear(&'a self, sx: Scalar, sy: Scalar) -> T {
        let shear = shear(sx, sy);
        self.transform(multiply(*self.get_transform(), shear))
    }
}

