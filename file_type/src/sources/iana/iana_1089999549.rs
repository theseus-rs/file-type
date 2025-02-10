use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1089999549: FileType = FileType {
    file_format: &FileFormat {
        id: 1_089_999_549,
        source_type: SourceType::Iana,
        name: "csv-schema",
        extensions: &[],
        media_types: &["text/csv-schema"],
        signatures: &[],
        related_formats: &[],
    },
};
