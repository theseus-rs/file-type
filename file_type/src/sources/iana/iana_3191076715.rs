use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3191076715: FileType = FileType {
    file_format: &FileFormat {
        id: 3_191_076_715,
        source_type: SourceType::Iana,
        name: "example",
        extensions: &[],
        media_types: &["message/example"],
        signatures: &[],
        related_formats: &[],
    },
};
