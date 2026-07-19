use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_681571915: FileType = FileType {
    file_format: &FileFormat {
        id: 681_571_915,
        source_type: SourceType::Iana,
        name: "vnd.cmmf-configuration-information+json",
        extensions: &[],
        media_types: &["application/vnd.cmmf-configuration-information+json"],
        signatures: &[],
        related_formats: &[],
    },
};
