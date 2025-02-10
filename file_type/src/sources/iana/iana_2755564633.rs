use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2755564633: FileType = FileType {
    file_format: &FileFormat {
        id: 2_755_564_633,
        source_type: SourceType::Iana,
        name: "shaclc",
        extensions: &[],
        media_types: &["text/shaclc"],
        signatures: &[],
        related_formats: &[],
    },
};
