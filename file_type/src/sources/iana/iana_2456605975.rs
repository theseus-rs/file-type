use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2456605975: FileType = FileType {
    file_format: &FileFormat {
        id: 2_456_605_975,
        source_type: SourceType::Iana,
        name: "stl",
        extensions: &[],
        media_types: &["model/stl"],
        signatures: &[],
        related_formats: &[],
    },
};
