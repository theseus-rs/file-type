use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_858598615: FileType = FileType {
    file_format: &FileFormat {
        id: 858_598_615,
        source_type: SourceType::Iana,
        name: "vnd.micro+json",
        extensions: &[],
        media_types: &["application/vnd.micro+json"],
        signatures: &[],
        related_formats: &[],
    },
};
