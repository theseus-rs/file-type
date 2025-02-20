use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1555320114: FileType = FileType {
    file_format: &FileFormat {
        id: 1_555_320_114,
        source_type: SourceType::Iana,
        name: "heif-sequence",
        extensions: &[],
        media_types: &["image/heif-sequence"],
        signatures: &[],
        related_formats: &[],
    },
};
