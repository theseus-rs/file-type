use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3079201304: FileType = FileType {
    file_format: &FileFormat {
        id: 3_079_201_304,
        source_type: SourceType::Iana,
        name: "vnd.oma-scws-http-response",
        extensions: &[],
        media_types: &["application/vnd.oma-scws-http-response"],
        signatures: &[],
        related_formats: &[],
    },
};
