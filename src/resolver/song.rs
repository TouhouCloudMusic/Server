use juniper::{FieldResult};
use entity::song;
use crate::model::input_model::{RandomSongInput, SongInput};
use crate::service::juniper::{JuniperContext};

pub struct SongQuery;
#[juniper::graphql_object]
#[graphql(context = JuniperContext)]
impl SongQuery {
    async fn song(
        input: SongInput,
        context: &JuniperContext,
    ) -> FieldResult<song::Model> {
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