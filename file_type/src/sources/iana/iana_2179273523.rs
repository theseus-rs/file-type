use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2179273523: FileType = FileType {
    file_format: &FileFormat {
        id: 2_179_273_523,
        source_type: SourceType::Iana,
        name: "vnd.hal+xml",
        extensions: &[],
        media_types: &["application/vnd.hal+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
