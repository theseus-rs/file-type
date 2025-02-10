use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_4077927361: FileType = FileType {
    file_format: &FileFormat {
        id: 4_077_927_361,
        source_type: SourceType::Iana,
        name: "ace-trl+cbor",
        extensions: &[],
        media_types: &["application/ace-trl+cbor"],
        signatures: &[],
        related_formats: &[],
    },
};
