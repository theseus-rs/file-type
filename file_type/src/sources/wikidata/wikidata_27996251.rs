use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27996251: FileType = FileType {
    file_format: &FileFormat {
        id: 27_996_251,
        source_type: SourceType::Wikidata,
        name: "InnoDB database file",
        extensions: &["ibd"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
