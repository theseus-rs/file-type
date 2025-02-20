use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_4059428465: FileType = FileType {
    file_format: &FileFormat {
        id: 4_059_428_465,
        source_type: SourceType::Iana,
        name: "vcard+json",
        extensions: &[],
        media_types: &["application/vcard+json"],
        signatures: &[],
        related_formats: &[],
    },
};
