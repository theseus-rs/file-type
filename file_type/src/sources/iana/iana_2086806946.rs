use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2086806946: FileType = FileType {
    file_format: &FileFormat {
        id: 2_086_806_946,
        source_type: SourceType::Iana,
        name: "index.obj",
        extensions: &[],
        media_types: &["application/index.obj"],
        signatures: &[],
        related_formats: &[],
    },
};
