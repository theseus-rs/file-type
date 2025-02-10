use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28207188: FileType = FileType {
    file_format: &FileFormat {
        id: 28_207_188,
        source_type: SourceType::Wikidata,
        name: "QDV",
        extensions: &["qdv"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
