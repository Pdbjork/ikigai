use crate::authorization::DocumentActionPermission;
use crate::background_job::storage_job::add_generate_waveform_job;
use async_graphql::*;
use uuid::Uuid;

use crate::db::file::File;
use crate::db::{Page, PageContent};
use crate::error::{IkigaiError, IkigaiErrorExt};
use crate::helper::{document_quick_authorize, get_conn_from_ctx};

#[derive(Default)]
pub struct FileQuery;

#[Object]
impl FileQuery {
    async fn get_file(&self, ctx: &Context<'_>, file_id: Uuid) -> Result<File> {
        let mut conn = get_conn_from_ctx(ctx).await?;
        let file = File::find_by_id(&mut conn, file_id).format_err()?;
        Ok(file)
    }

    async fn file_waveform(
        &self,
        ctx: &Context<'_>,
        file_id: Uuid,
        document_id: Uuid,
    ) -> Result<Option<String>> {
        document_quick_authorize(ctx, document_id, DocumentActionPermission::ViewDocument).await?;

        let mut conn = get_conn_from_ctx(ctx).await?;
        
        let pages = Page::find_all_by_document_id(&mut conn, document_id).format_err()?;
        let page_ids = pages.iter().map(|p| p.id).collect();
        let page_contents = PageContent::find_all_by_pages(&mut conn, page_ids).format_err()?;
        
        let is_file_in_document = page_contents.iter().any(|content| {
            content.get_json_content().has_file_handler(file_id)
        });

        if !is_file_in_document {
             return Err(IkigaiError::new_bad_request("File not found in document")).format_err();
        }

        let file = File::find_by_id(&mut conn, file_id).format_err()?;
        let is_mp3_file = file.content_type == "audio/mpeg";
        if is_mp3_file && file.waveform_audio_json_str.is_none() {
            add_generate_waveform_job(file.uuid);
        }

        Ok(file.waveform_audio_json_str)
    }
}
