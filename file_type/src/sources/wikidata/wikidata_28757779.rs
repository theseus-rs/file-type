use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28757779: FileType = FileType {
    file_format: &FileFormat {
        id: 28_757_779,
        source_type: SourceType::Wikidata,
        name: "GME",
        extensions: &["gme"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
