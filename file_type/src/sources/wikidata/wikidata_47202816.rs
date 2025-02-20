use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_47202816: FileType = FileType {
    file_format: &FileFormat {
        id: 47_202_816,
        source_type: SourceType::Wikidata,
        name: "AppleWorks Painting file format version 6",
        extensions: &["cwk"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
