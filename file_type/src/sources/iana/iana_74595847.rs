use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_74595847: FileType = FileType {
    file_format: &FileFormat {
        id: 74_595_847,
        source_type: SourceType::Iana,
        name: "bmp",
        extensions: &[],
        media_types: &["image/bmp"],
        signatures: &[],
        related_formats: &[],
    },
};
