use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1660650536: FileType = FileType {
    file_format: &FileFormat {
        id: 1_660_650_536,
        source_type: SourceType::Iana,
        name: "rtx",
        extensions: &[],
        media_types: &["application/rtx"],
        signatures: &[],
        related_formats: &[],
    },
};
