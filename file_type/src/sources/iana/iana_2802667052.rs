use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2802667052: FileType = FileType {
    file_format: &FileFormat {
        id: 2_802_667_052,
        source_type: SourceType::Iana,
        name: "vnd.ieee.1905",
        extensions: &[],
        media_types: &["application/vnd.ieee.1905"],
        signatures: &[],
        related_formats: &[],
    },
};
