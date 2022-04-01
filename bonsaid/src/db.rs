use bonsaidb::core::document::{CollectionDocument, Emit};
use bonsaidb::core::schema::view::CollectionViewSchema;
use bonsaidb::core::schema::{Collection, ReduceResult, View, ViewMapResult, ViewMappedValue};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Collection)]
#[collection(name = "posts", views = [PostsCount])]
pub struct Post {
    pub title: String,
}

impl Default for Post {
    fn default() -> Self {
        Self {
            title: "Untitled".to_owned(),
        }
    }
}

#[derive(Debug, Clone, View)]
#[view(collection = Post, key = u64, value = usize, name = "posts-count")]
pub struct PostsCount;

impl CollectionViewSchema for PostsCount {
    type View = Self;

    fn map(&self, document: CollectionDocument<Post>) -> ViewMapResult<Self::View> {
        document.header.emit_key_and_value(document.header.id, 1)
    }

    fn reduce(
        &self,
        mappings: &[ViewMappedValue<Self>],
        _rereduce: bool,
    ) -> ReduceResult<Self::View> {
        Ok(mappings.iter().map(|m| m.value).sum())
    }
}

// * TESTS

#[allow(dead_code)]
fn create_post_db() -> Result<bonsaidb::local::Database, bonsaidb::core::Error> {
    use bonsaidb::local::{
        config::{Builder, StorageConfiguration},
        Database,
    };

    let db = Database::open::<Post>(StorageConfiguration::new("").memory_only())?;

    Ok(db)
}

#[test]
fn can_create_post_db() {
    create_post_db().unwrap();
}

#[test]
fn can_create_posts() -> Result<(), bonsaidb::core::Error> {
    use bonsaidb::core::schema::SerializedCollection;
    let db = create_post_db()?;

    let post = Post::default().push_into(&db)?;

    let db_post = Post::get(post.header.id, &db)?.expect("couldn't retrieve stored item");

    assert_eq!(db_post.header.id, post.header.id);

    Ok(())
}

#[test]
fn can_update_posts() -> Result<(), bonsaidb::core::Error> {
    use bonsaidb::core::schema::SerializedCollection;
    let db = create_post_db()?;

    let mut post = Post::default().push_into(&db)?;

    post.contents.title = "Updated".to_owned();

    assert_eq!(post.contents.title, "Updated");

    Ok(())
}

#[test]
fn can_delete_posts() -> Result<(), bonsaidb::core::Error> {
    use bonsaidb::core::schema::SerializedCollection;
    let db = create_post_db()?;

    let post = Post::default().push_into(&db)?;

    post.delete(&db)?;

    Ok(())
}

#[test]
fn can_crud_posts () -> Result<(), bonsaidb::core::Error> {
    // TODO: Impl
    Ok(())
}
