use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_109944694: FileType = FileType {
    file_format: &FileFormat {
        id: 109_944_694,
        source_type: SourceType::Wikidata,
        name: "Archives file format",
        extensions: &["arv"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
