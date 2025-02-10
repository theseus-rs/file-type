use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_837519450: FileType = FileType {
    file_format: &FileFormat {
        id: 837_519_450,
        source_type: SourceType::Iana,
        name: "dpop+jwt",
        extensions: &[],
        media_types: &["application/dpop+jwt"],
        signatures: &[],
        related_formats: &[],
    },
};
