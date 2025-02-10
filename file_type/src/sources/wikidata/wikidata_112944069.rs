use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_112944069: FileType = FileType {
    file_format: &FileFormat {
        id: 112_944_069,
        source_type: SourceType::Wikidata,
        name: "GameExchange2 animation file",
        extensions: &["GAF"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
