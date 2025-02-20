use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3930646908: FileType = FileType {
    file_format: &FileFormat {
        id: 3_930_646_908,
        source_type: SourceType::Iana,
        name: "example",
        extensions: &[],
        media_types: &["application/example"],
        signatures: &[],
        related_formats: &[],
    },
};
