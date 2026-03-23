/// Minimum page cursor
pub const PAGE_CURSOR_MIN: Cursor = 1;

/// Maximum page cursor
pub const PAGE_CURSOR_MAX: Cursor = 999_999_999_900;

/// Minimum page limit
pub const PAGE_LIMIT_MIN: Limit = 10;

/// Maximum page limit
pub const PAGE_LIMIT_MAX: Limit = 100;

/// Type alias for page cursor
pub type Cursor = i64;

/// Type alias for page size limit
pub type Limit = i64;

/// A cursor position and data.
pub struct Page<T>(pub Cursor, pub Vec<T>);

/// A cursor position and size limit.
pub struct PageParams(pub Cursor, pub Limit);

/// Sets some reasonable defaults for page parameters.
impl Default for PageParams {
    fn default() -> Self {
        Self(PAGE_CURSOR_MIN, PAGE_LIMIT_MIN)
    }
}
