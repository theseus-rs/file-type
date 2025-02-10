use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3814590938: FileType = FileType {
    file_format: &FileFormat {
        id: 3_814_590_938,
        source_type: SourceType::Iana,
        name: "vnd.3gpp.seal-info+xml",
        extensions: &[],
        media_types: &["application/vnd.3gpp.seal-info+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
