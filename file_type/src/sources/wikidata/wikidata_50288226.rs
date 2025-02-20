use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_50288226: FileType = FileType {
    file_format: &FileFormat {
        id: 50_288_226,
        source_type: SourceType::Wikidata,
        name: "Adobe Air, v1.5",
        extensions: &["air"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
