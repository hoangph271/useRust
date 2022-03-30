use bonsaidb::core::document::{CollectionDocument, Emit};
use bonsaidb::core::schema::view::CollectionViewSchema;
use bonsaidb::core::schema::{Collection, ReduceResult, View, ViewMapResult, ViewMappedValue};
use bonsaidb::local::{
    config::{Builder, StorageConfiguration},
    Database,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Collection)]
#[collection(name = "shapes", views = [PostsCount])]
pub struct Post {
    pub id: u32,
    pub title: String,
}

#[derive(Debug, Clone, View)]
#[view(collection = Post, key = u32, value = usize, name = "posts-count")]
pub struct PostsCount;

impl CollectionViewSchema for PostsCount {
    type View = Self;

    fn map(&self, document: CollectionDocument<Post>) -> ViewMapResult<Self::View> {
        document.header.emit_key_and_value(document.contents.id, 1)
    }

    fn reduce(
        &self,
        mappings: &[ViewMappedValue<Self>],
        _rereduce: bool,
    ) -> ReduceResult<Self::View> {
        Ok(mappings.iter().map(|m| m.value).sum())
    }
}

#[test]
fn test_can_run() {}

pub async fn can_create_posts() -> Result<(), bonsaidb::core::Error> {
    let db = Database::open::<Post>(StorageConfiguration::new("view-examples.bonsaidb")).await;

    Ok(())
}
