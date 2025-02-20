use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1902469027: FileType = FileType {
    file_format: &FileFormat {
        id: 1_902_469_027,
        source_type: SourceType::Iana,
        name: "JT",
        extensions: &[],
        media_types: &["model/JT"],
        signatures: &[],
        related_formats: &[],
    },
};
