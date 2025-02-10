use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28206105: FileType = FileType {
    file_format: &FileFormat {
        id: 28_206_105,
        source_type: SourceType::Wikidata,
        name: "Falcon True Color",
        extensions: &["ftc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
