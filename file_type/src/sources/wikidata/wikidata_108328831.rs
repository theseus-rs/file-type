use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_108328831: FileType = FileType {
    file_format: &FileFormat {
        id: 108_328_831,
        source_type: SourceType::Wikidata,
        name: "Universe Sandbox Data File",
        extensions: &["ubox"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
