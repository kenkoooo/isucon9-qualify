use crate::IsuResult;
use actix_files::NamedFile;

pub async fn get_index() -> IsuResult<NamedFile> {
    Ok(NamedFile::open("../public/index.html")?)
}
