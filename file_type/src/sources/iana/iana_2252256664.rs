use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2252256664: FileType = FileType {
    file_format: &FileFormat {
        id: 2_252_256_664,
        source_type: SourceType::Iana,
        name: "scim+json",
        extensions: &[],
        media_types: &["application/scim+json"],
        signatures: &[],
        related_formats: &[],
    },
};
