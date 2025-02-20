use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1337992840: FileType = FileType {
    file_format: &FileFormat {
        id: 1_337_992_840,
        source_type: SourceType::Iana,
        name: "vnd.mix",
        extensions: &[],
        media_types: &["image/vnd.mix"],
        signatures: &[],
        related_formats: &[],
    },
};
