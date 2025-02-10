use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2358701609: FileType = FileType {
    file_format: &FileFormat {
        id: 2_358_701_609,
        source_type: SourceType::Iana,
        name: "its+xml",
        extensions: &[],
        media_types: &["application/its+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
