use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3972709196: FileType = FileType {
    file_format: &FileFormat {
        id: 3_972_709_196,
        source_type: SourceType::Iana,
        name: "vnd.adobe.xdp+xml",
        extensions: &[],
        media_types: &["application/vnd.adobe.xdp+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
