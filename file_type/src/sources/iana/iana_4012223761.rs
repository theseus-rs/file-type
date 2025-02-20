use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_4012223761: FileType = FileType {
    file_format: &FileFormat {
        id: 4_012_223_761,
        source_type: SourceType::Iana,
        name: "hl7v2+xml",
        extensions: &[],
        media_types: &["application/hl7v2+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
