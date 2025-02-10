use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2215690392: FileType = FileType {
    file_format: &FileFormat {
        id: 2_215_690_392,
        source_type: SourceType::Iana,
        name: "widget",
        extensions: &[],
        media_types: &["application/widget"],
        signatures: &[],
        related_formats: &[],
    },
};
