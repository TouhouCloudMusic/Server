use juniper::{FieldResult};
use entity::song;
use crate::model::input_model::{CreateSongInput, RandomSongInput, RetrieveSongInput};
use crate::service::juniper::{JuniperContext};

pub struct SongQuery;
pub struct SongMutation;

#[juniper::graphql_object]
#[graphql(context = JuniperContext)]
impl SongQuery {
    async fn retrieve(
        input: RetrieveSongInput,
        context: &JuniperContext,
    ) -> FieldResult<Option<song::Model>> {
        let song_service = &context.song_service;
        let song = song_service
            .find_by_id(input.id)
            .await?;

        Ok(song)
    }

    async fn random(
        input: RandomSongInput,
        context: &JuniperContext,
    ) -> FieldResult<Vec<song::Model>> {
        let song_service = &context.song_service;
        let song = song_service.random(input.count as u64)
            .await?;

        Ok(song)
    }
}

#[juniper::graphql_object]
#[graphql(context = JuniperContext)]
impl SongMutation {
    async fn create(
        input: CreateSongInput,
        context: &JuniperContext,
    ) -> FieldResult<song::Model> {
        let song_service = &context.song_service;
        let new_song = song_service.create(
            input.status,
            input.title,
            input.created_at,
            input.updated_at,
        ).await?;
        
        Ok(new_song)
    }
}