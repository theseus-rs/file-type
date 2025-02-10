use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_372858196: FileType = FileType {
    file_format: &FileFormat {
        id: 372_858_196,
        source_type: SourceType::Iana,
        name: "shf+xml",
        extensions: &[],
        media_types: &["application/shf+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
