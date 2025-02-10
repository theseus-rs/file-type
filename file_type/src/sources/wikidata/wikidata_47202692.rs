use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_47202692: FileType = FileType {
    file_format: &FileFormat {
        id: 47_202_692,
        source_type: SourceType::Wikidata,
        name: "AppleWorks Database file format version 6",
        extensions: &["cwk"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
