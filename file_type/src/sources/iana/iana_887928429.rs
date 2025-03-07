use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_887928429: FileType = FileType {
    file_format: &FileFormat {
        id: 887_928_429,
        source_type: SourceType::Iana,
        name: "vnd.clip",
        extensions: &[],
        media_types: &["image/vnd.clip"],
        signatures: &[],
        related_formats: &[],
    },
};
