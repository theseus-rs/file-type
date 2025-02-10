use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_2328734: FileType = FileType {
    file_format: &FileFormat {
        id: 2_328_734,
        source_type: SourceType::Wikidata,
        name: "JISP",
        extensions: &["jisp"],
        media_types: &["application/vnd.jisp"],
        signatures: &[],
        related_formats: &[],
    },
};
