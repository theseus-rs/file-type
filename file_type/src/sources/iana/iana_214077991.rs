use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_214077991: FileType = FileType {
    file_format: &FileFormat {
        id: 214_077_991,
        source_type: SourceType::Iana,
        name: "j2c",
        extensions: &[],
        media_types: &["image/j2c"],
        signatures: &[],
        related_formats: &[],
    },
};
