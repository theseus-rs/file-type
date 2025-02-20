use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_114078864: FileType = FileType {
    file_format: &FileFormat {
        id: 114_078_864,
        source_type: SourceType::Wikidata,
        name: "MacBinary II",
        extensions: &["bin"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
