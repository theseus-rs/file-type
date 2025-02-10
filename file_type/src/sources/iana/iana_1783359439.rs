use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1783359439: FileType = FileType {
    file_format: &FileFormat {
        id: 1_783_359_439,
        source_type: SourceType::Iana,
        name: "wita",
        extensions: &[],
        media_types: &["application/wita"],
        signatures: &[],
        related_formats: &[],
    },
};
