use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2984449870: FileType = FileType {
    file_format: &FileFormat {
        id: 2_984_449_870,
        source_type: SourceType::Iana,
        name: "vnd.sar",
        extensions: &[],
        media_types: &["application/vnd.sar"],
        signatures: &[],
        related_formats: &[],
    },
};
