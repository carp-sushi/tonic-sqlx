use super::Story;

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

/// Page parameters: cursor position and page size limit.
pub struct PageParams(pub Cursor, pub Limit);

/// A page of stories: next cursor and the story list.
pub struct StoryPage(pub Cursor, pub Vec<Story>);

/// Sets some reasonable defaults for page parameters.
impl Default for PageParams {
    fn default() -> Self {
        Self(PAGE_CURSOR_MIN, PAGE_LIMIT_MIN)
    }
}
