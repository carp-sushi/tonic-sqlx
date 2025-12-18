use crate::{
    Result,
    domain::{Story, StoryId},
    repo::Repo,
    usecase::UseCase,
    usecase::story::{CreateStory, DeleteStory, ListStories, UpdateStory},
};
use std::sync::Arc;

/// Story service
pub struct StoryService {
    create_story: CreateStory,
    list_stories: ListStories,
    delete_story: DeleteStory,
    update_story: UpdateStory,
}

impl StoryService {
    /// Constructor
    pub fn new(repo: Arc<Repo>) -> Self {
        Self {
            create_story: CreateStory::new(repo.clone()),
            list_stories: ListStories::new(repo.clone()),
            delete_story: DeleteStory::new(repo.clone()),
            update_story: UpdateStory::new(repo),
        }
    }

    /// Fetch a page of stories
    pub async fn list_stories(&self, page_params: (i64, i64)) -> Result<(i64, Vec<Story>)> {
        self.list_stories.execute(page_params).await
    }

    /// Create a new story
    pub async fn create_story(&self, name: String) -> Result<Story> {
        self.create_story.execute(name).await
    }

    /// Update an existing story
    pub async fn update_story(&self, story_id: StoryId, name: String) -> Result<Story> {
        self.update_story
            .execute(UpdateStory::args(story_id, name))
            .await
    }

    /// Delete an existing story
    pub async fn delete_story(&self, story_id: StoryId) -> Result<()> {
        self.delete_story.execute(story_id).await
    }
}
