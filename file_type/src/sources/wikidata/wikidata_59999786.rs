use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_59999786: FileType = FileType {
    file_format: &FileFormat {
        id: 59_999_786,
        source_type: SourceType::Wikidata,
        name: "Dreamweaver Lock File",
        extensions: &["lck"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
