use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1061990033: FileType = FileType {
    file_format: &FileFormat {
        id: 1_061_990_033,
        source_type: SourceType::Iana,
        name: "mosskey-data",
        extensions: &[],
        media_types: &["application/mosskey-data"],
        signatures: &[],
        related_formats: &[],
    },
};
