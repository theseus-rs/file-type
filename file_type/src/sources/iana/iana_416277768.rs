use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_416277768: FileType = FileType {
    file_format: &FileFormat {
        id: 416_277_768,
        source_type: SourceType::Iana,
        name: "vnd.zoho-document.writer",
        extensions: &[],
        media_types: &["application/vnd.zoho-document.writer"],
        signatures: &[],
        related_formats: &[],
    },
};
