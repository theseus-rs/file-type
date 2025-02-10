use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1416691203: FileType = FileType {
    file_format: &FileFormat {
        id: 1_416_691_203,
        source_type: SourceType::Iana,
        name: "CDFX+XML",
        extensions: &[],
        media_types: &["application/CDFX+XML"],
        signatures: &[],
        related_formats: &[],
    },
};
