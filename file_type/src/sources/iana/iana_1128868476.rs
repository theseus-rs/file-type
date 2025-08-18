use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1128868476: FileType = FileType {
    file_format: &FileFormat {
        id: 1_128_868_476,
        source_type: SourceType::Iana,
        name: "json-patch-query+json",
        extensions: &[],
        media_types: &["application/json-patch-query+json"],
        signatures: &[],
        related_formats: &[],
    },
};
