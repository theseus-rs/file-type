use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_345349413: FileType = FileType {
    file_format: &FileFormat {
        id: 345_349_413,
        source_type: SourceType::Iana,
        name: "vnd.powerbuilder6-s",
        extensions: &[],
        media_types: &["application/vnd.powerbuilder6-s"],
        signatures: &[],
        related_formats: &[],
    },
};
