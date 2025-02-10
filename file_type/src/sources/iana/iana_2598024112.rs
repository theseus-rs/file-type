use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2598024112: FileType = FileType {
    file_format: &FileFormat {
        id: 2_598_024_112,
        source_type: SourceType::Iana,
        name: "rtploopback",
        extensions: &[],
        media_types: &["video/rtploopback"],
        signatures: &[],
        related_formats: &[],
    },
};
