use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2775591539: FileType = FileType {
    file_format: &FileFormat {
        id: 2_775_591_539,
        source_type: SourceType::Iana,
        name: "vnd.ecdis-update",
        extensions: &[],
        media_types: &["application/vnd.ecdis-update"],
        signatures: &[],
        related_formats: &[],
    },
};
