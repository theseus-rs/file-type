use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_197872702: FileType = FileType {
    file_format: &FileFormat {
        id: 197_872_702,
        source_type: SourceType::Iana,
        name: "atsc-held+xml",
        extensions: &[],
        media_types: &["application/atsc-held+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
