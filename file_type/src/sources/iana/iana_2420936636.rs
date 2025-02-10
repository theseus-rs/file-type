use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2420936636: FileType = FileType {
    file_format: &FileFormat {
        id: 2_420_936_636,
        source_type: SourceType::Iana,
        name: "provided-claims+jwt",
        extensions: &[],
        media_types: &["application/provided-claims+jwt"],
        signatures: &[],
        related_formats: &[],
    },
};
