use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27979542: FileType = FileType {
    file_format: &FileFormat {
        id: 27_979_542,
        source_type: SourceType::Wikidata,
        name: "BookmarkData",
        extensions: &["sfl"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
