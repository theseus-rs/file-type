use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2510594029: FileType = FileType {
    file_format: &FileFormat {
        id: 2_510_594_029,
        source_type: SourceType::Iana,
        name: "vnd.cxzip",
        extensions: &[],
        media_types: &["application/vnd.cxzip"],
        signatures: &[],
        related_formats: &[],
    },
};
