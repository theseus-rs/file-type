use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2806789601: FileType = FileType {
    file_format: &FileFormat {
        id: 2_806_789_601,
        source_type: SourceType::Iana,
        name: "vnd.poc.group-advertisement+xml",
        extensions: &[],
        media_types: &["application/vnd.poc.group-advertisement+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
