use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3848466485: FileType = FileType {
    file_format: &FileFormat {
        id: 3_848_466_485,
        source_type: SourceType::Iana,
        name: "font-sfnt - DEPRECATED in favor of font/sfnt",
        extensions: &[],
        media_types: &["application/font-sfnt"],
        signatures: &[],
        related_formats: &[],
    },
};
