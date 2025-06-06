use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_47197074: FileType = FileType {
    file_format: &FileFormat {
        id: 47_197_074,
        source_type: SourceType::Wikidata,
        name: "AppleWorks Drawing file format version 6",
        extensions: &["cwk"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
