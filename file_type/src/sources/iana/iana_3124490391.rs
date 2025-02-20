use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3124490391: FileType = FileType {
    file_format: &FileFormat {
        id: 3_124_490_391,
        source_type: SourceType::Iana,
        name: "xml-external-parsed-entity",
        extensions: &[],
        media_types: &["application/xml-external-parsed-entity"],
        signatures: &[],
        related_formats: &[],
    },
};
