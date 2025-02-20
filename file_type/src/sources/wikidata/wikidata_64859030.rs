use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_64859030: FileType = FileType {
    file_format: &FileFormat {
        id: 64_859_030,
        source_type: SourceType::Wikidata,
        name: "Family Tree Maker for Windows file format",
        extensions: &["ftw"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
