use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3439810779: FileType = FileType {
    file_format: &FileFormat {
        id: 3_439_810_779,
        source_type: SourceType::Iana,
        name: "toml",
        extensions: &[],
        media_types: &["application/toml"],
        signatures: &[],
        related_formats: &[],
    },
};
