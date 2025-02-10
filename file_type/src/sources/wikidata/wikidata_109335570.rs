use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_109335570: FileType = FileType {
    file_format: &FileFormat {
        id: 109_335_570,
        source_type: SourceType::Wikidata,
        name: "comic book archive, ZIP container",
        extensions: &["cbz"],
        media_types: &["application/vnd.comicbook+cbz"],
        signatures: &[],
        related_formats: &[],
    },
};
