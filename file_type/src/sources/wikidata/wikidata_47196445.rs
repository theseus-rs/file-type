use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_47196445: FileType = FileType {
    file_format: &FileFormat {
        id: 47_196_445,
        source_type: SourceType::Wikidata,
        name: "AppleWorks Database file format, version 5",
        extensions: &["cwk"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
