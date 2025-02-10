use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_70571009: FileType = FileType {
    file_format: &FileFormat {
        id: 70_571_009,
        source_type: SourceType::Iana,
        name: "dssc+der",
        extensions: &[],
        media_types: &["application/dssc+der"],
        signatures: &[],
        related_formats: &[],
    },
};
