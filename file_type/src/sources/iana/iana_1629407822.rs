use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1629407822: FileType = FileType {
    file_format: &FileFormat {
        id: 1_629_407_822,
        source_type: SourceType::Iana,
        name: "example",
        extensions: &[],
        media_types: &["model/example"],
        signatures: &[],
        related_formats: &[],
    },
};
