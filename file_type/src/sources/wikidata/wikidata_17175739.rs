use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_17175739: FileType = FileType {
    file_format: &FileFormat {
        id: 17_175_739,
        source_type: SourceType::Wikidata,
        name: "comic book archive, RAR container",
        extensions: &["cbr"],
        media_types: &["application/vnd.comicbook-rar"],
        signatures: &[],
        related_formats: &[],
    },
};
