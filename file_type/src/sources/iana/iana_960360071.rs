use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_960360071: FileType = FileType {
    file_format: &FileFormat {
        id: 960_360_071,
        source_type: SourceType::Iana,
        name: "vnd.hyper-item+json",
        extensions: &[],
        media_types: &["application/vnd.hyper-item+json"],
        signatures: &[],
        related_formats: &[],
    },
};
