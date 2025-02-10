use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_376985049: FileType = FileType {
    file_format: &FileFormat {
        id: 376_985_049,
        source_type: SourceType::Iana,
        name: "scaip+xml",
        extensions: &[],
        media_types: &["application/scaip+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
