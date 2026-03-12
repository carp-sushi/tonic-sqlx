mod status;
mod story;
mod task;

pub use status::Status;
pub use story::{Story, StoryId};
pub use task::{Task, TaskId};

/// Type alias for page cursor
pub type Cursor = i64;

/// Type alias for page size limit
pub type Limit = i64;

/// Type alias for page parameters
pub type PageParams = (Cursor, Limit);

/// Type alias for page of stories
pub type StoryPage = (Cursor, Vec<Story>);
