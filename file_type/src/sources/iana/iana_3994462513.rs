use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3994462513: FileType = FileType {
    file_format: &FileFormat {
        id: 3_994_462_513,
        source_type: SourceType::Iana,
        name: "font-woff - DEPRECATED in favor of font/woff",
        extensions: &[],
        media_types: &["application/font-woff"],
        signatures: &[],
        related_formats: &[],
    },
};
