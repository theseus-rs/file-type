use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_66439286: FileType = FileType {
    file_format: &FileFormat {
        id: 66_439_286,
        source_type: SourceType::Wikidata,
        name: "DisplayWrite Document file format, version 5",
        extensions: &["doc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
