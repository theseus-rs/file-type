use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_503630353: FileType = FileType {
    file_format: &FileFormat {
        id: 503_630_353,
        source_type: SourceType::Iana,
        name: "mp21",
        extensions: &[],
        media_types: &["application/mp21"],
        signatures: &[],
        related_formats: &[],
    },
};
