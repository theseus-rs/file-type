use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_64859082: FileType = FileType {
    file_format: &FileFormat {
        id: 64_859_082,
        source_type: SourceType::Wikidata,
        name: "Family Tree Maker for DOS file format",
        extensions: &["ftm"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
