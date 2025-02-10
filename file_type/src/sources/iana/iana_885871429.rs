use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_885871429: FileType = FileType {
    file_format: &FileFormat {
        id: 885_871_429,
        source_type: SourceType::Iana,
        name: "vnd.etsi.sci+xml",
        extensions: &[],
        media_types: &["application/vnd.etsi.sci+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
