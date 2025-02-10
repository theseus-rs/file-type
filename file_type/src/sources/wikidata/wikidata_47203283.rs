use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_47203283: FileType = FileType {
    file_format: &FileFormat {
        id: 47_203_283,
        source_type: SourceType::Wikidata,
        name: "AppleWorks Presentation file format",
        extensions: &["cwk"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
