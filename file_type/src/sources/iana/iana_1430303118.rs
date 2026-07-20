use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1430303118: FileType = FileType {
    file_format: &FileFormat {
        id: 1_430_303_118,
        source_type: SourceType::Iana,
        name: "vnd.agtp.identity+json",
        extensions: &[],
        media_types: &["application/vnd.agtp.identity+json"],
        signatures: &[],
        related_formats: &[],
    },
};
