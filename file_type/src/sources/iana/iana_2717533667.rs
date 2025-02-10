use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2717533667: FileType = FileType {
    file_format: &FileFormat {
        id: 2_717_533_667,
        source_type: SourceType::Iana,
        name: "example",
        extensions: &[],
        media_types: &["text/example"],
        signatures: &[],
        related_formats: &[],
    },
};
