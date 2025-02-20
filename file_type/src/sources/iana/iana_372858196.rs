use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
