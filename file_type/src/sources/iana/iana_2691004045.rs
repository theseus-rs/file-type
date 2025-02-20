use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2691004045: FileType = FileType {
    file_format: &FileFormat {
        id: 2_691_004_045,
        source_type: SourceType::Iana,
        name: "eshop",
        extensions: &[],
        media_types: &["application/eshop"],
        signatures: &[],
        related_formats: &[],
    },
};
