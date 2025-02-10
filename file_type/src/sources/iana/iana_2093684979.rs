use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2093684979: FileType = FileType {
    file_format: &FileFormat {
        id: 2_093_684_979,
        source_type: SourceType::Iana,
        name: "vnd.avistar+xml",
        extensions: &[],
        media_types: &["application/vnd.avistar+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
