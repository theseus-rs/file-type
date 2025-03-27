use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_114048106: FileType = FileType {
    file_format: &FileFormat {
        id: 114_048_106,
        source_type: SourceType::Wikidata,
        name: "Apple Partition Map ISO 9660 Hybrid",
        extensions: &["cdr", "iso", "toast"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
