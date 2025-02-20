use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
