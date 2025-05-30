use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_112652551: FileType = FileType {
    file_format: &FileFormat {
        id: 112_652_551,
        source_type: SourceType::Wikidata,
        name: "Astound Actor file format",
        extensions: &["act"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
