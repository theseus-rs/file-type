use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_617077046: FileType = FileType {
    file_format: &FileFormat {
        id: 617_077_046,
        source_type: SourceType::Iana,
        name: "vnd.ms-htmlhelp",
        extensions: &[],
        media_types: &["application/vnd.ms-htmlhelp"],
        signatures: &[],
        related_formats: &[],
    },
};
