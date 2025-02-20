use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
