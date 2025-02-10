use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1400753887: FileType = FileType {
    file_format: &FileFormat {
        id: 1_400_753_887,
        source_type: SourceType::Iana,
        name: "css",
        extensions: &[],
        media_types: &["text/css"],
        signatures: &[],
        related_formats: &[],
    },
};
