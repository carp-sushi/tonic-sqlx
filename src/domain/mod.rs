mod effect;
mod status;
mod story;
mod task;

pub use effect::{StoryEffects, TaskEffects};
pub use status::Status;
pub use story::{Story, StoryId};
pub use task::{Task, TaskId};

/// Type alias for page cursor
pub type Cursor = i64;

/// Type alias for page size limit
pub type Limit = i64;

/// Type for page paramaters (for readability)
pub type PageParams = (Cursor, Limit);
