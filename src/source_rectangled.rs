use math::relative_source_rectangle;
use types::SourceRectangle;

/// Should be implemented by contexts that
/// have source rectangle information.
pub trait SourceRectangled {
    /// Adds a source rectangle.
    fn src_rect(self, x: i32, y: i32, w: i32, h: i32) -> Self;

    /// Moves to a relative source rectangle using
    /// the current source rectangle as tile.
    fn src_rel(self, x: i32, y: i32) -> Self;

    /// Flips the source rectangle horizontally.
    fn src_flip_h(self) -> Self;

    /// Flips the source rectangle vertically.
    fn src_flip_v(self) -> Self;

    /// Flips the source rectangle horizontally and vertically.
    fn src_flip_hv(self) -> Self;
}

impl SourceRectangled for SourceRectangle {
    #[inline(always)]
    fn src_rect(self, x: i32, y: i32, w: i32, h: i32) -> Self {
        [x, y, w, h]
    }

    #[inline(always)]
    fn src_rel(self, x: i32, y: i32) -> Self {
        relative_source_rectangle(self, x, y)
    }

    #[inline(always)]
    fn src_flip_h(self) -> Self {
        [
            self[0] + self[2],
            self[1],
            -self[2],
            self[3]
        ]
    }

    #[inline(always)]
    fn src_flip_v(self) -> Self {
        [
            self[0],
            self[1] + self[3],
            self[2],
            -self[3]
        ]
    }

    #[inline(always)]
    fn src_flip_hv(self) -> Self {
        [
            self[0] + self[2],
            self[1] + self[3],
            -self[2],
            -self[3]
        ]
    }
}
