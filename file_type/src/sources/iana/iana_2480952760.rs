use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2480952760: FileType = FileType {
    file_format: &FileFormat {
        id: 2_480_952_760,
        source_type: SourceType::Iana,
        name: "eat-bun+json",
        extensions: &[],
        media_types: &["application/eat-bun+json"],
        signatures: &[],
        related_formats: &[],
    },
};
