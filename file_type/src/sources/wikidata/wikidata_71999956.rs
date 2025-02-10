use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_71999956: FileType = FileType {
    file_format: &FileFormat {
        id: 71_999_956,
        source_type: SourceType::Wikidata,
        name: "iTunes Cover Flow Data file format, version 2",
        extensions: &["itc2"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
