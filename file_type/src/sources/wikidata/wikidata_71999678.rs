use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_71999678: FileType = FileType {
    file_format: &FileFormat {
        id: 71_999_678,
        source_type: SourceType::Wikidata,
        name: "iTunes CoverFlow data file format",
        extensions: &["itc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
