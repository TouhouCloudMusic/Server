use crate::model::input_model::RetrieveReleaseInput;
use crate::service::juniper::JuniperContext;
use entity::release;
use juniper::FieldResult;

pub struct ReleaseQuery;
// pub struct ReleaseMutation;

#[juniper::graphql_object]
#[graphql(context = JuniperContext)]
impl ReleaseQuery {
    async fn retrieve(
        input: RetrieveReleaseInput,
        context: &JuniperContext,
    ) -> FieldResult<Option<release::Model>> {
        let release_service = &context.release_service;
        let release = release_service.find_by_id(input.id).await?;

        Ok(release)
    }
}

// #[juniper::graphql_object]
// #[graphql(context = JuniperContext)]
// impl ReleaseMutation {
//     async fn create(
//         input: CreateSongInput,
//         context: &JuniperContext,
//     ) -> FieldResult<song::Model> {
//         let song_service = &context.song_service;
//         let new_song = song_service.create(
//             input.status,
//             input.title,
//             input.created_at,
//             input.updated_at,
//         ).await?;
//
//         Ok(new_song)
//     }
// }
