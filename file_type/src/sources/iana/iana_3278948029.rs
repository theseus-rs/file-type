use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3278948029: FileType = FileType {
    file_format: &FileFormat {
        id: 3_278_948_029,
        source_type: SourceType::Iana,
        name: "vnd.fujitsu.oasysprs",
        extensions: &[],
        media_types: &["application/vnd.fujitsu.oasysprs"],
        signatures: &[],
        related_formats: &[],
    },
};
