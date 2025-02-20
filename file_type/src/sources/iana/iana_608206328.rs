use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_608206328: FileType = FileType {
    file_format: &FileFormat {
        id: 608_206_328,
        source_type: SourceType::Iana,
        name: "alto-updatestreamcontrol+json",
        extensions: &[],
        media_types: &["application/alto-updatestreamcontrol+json"],
        signatures: &[],
        related_formats: &[],
    },
};
