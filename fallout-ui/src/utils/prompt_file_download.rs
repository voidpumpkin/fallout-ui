use crate::utils::js;
use crate::utils::web_error::WebResult;

use super::js::DownloadedFile;

pub fn prompt_file_download(file: &DownloadedFile) -> WebResult<()> {
    Ok(js::prompt_file_download(file)?)
}
