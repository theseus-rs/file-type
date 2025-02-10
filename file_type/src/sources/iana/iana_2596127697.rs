use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2596127697: FileType = FileType {
    file_format: &FileFormat {
        id: 2_596_127_697,
        source_type: SourceType::Iana,
        name: "vnd.rapid",
        extensions: &[],
        media_types: &["application/vnd.rapid"],
        signatures: &[],
        related_formats: &[],
    },
};
