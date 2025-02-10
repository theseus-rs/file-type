use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_376520231: FileType = FileType {
    file_format: &FileFormat {
        id: 376_520_231,
        source_type: SourceType::Iana,
        name: "atsc-dwd+xml",
        extensions: &[],
        media_types: &["application/atsc-dwd+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
