use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1132854721: FileType = FileType {
    file_format: &FileFormat {
        id: 1_132_854_721,
        source_type: SourceType::Iana,
        name: "avci",
        extensions: &[],
        media_types: &["image/avci"],
        signatures: &[],
        related_formats: &[],
    },
};
