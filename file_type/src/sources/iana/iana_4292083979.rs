use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_4292083979: FileType = FileType {
    file_format: &FileFormat {
        id: 4_292_083_979,
        source_type: SourceType::Iana,
        name: "provenance+xml",
        extensions: &[],
        media_types: &["application/provenance+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
