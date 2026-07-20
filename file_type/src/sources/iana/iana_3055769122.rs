use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3055769122: FileType = FileType {
    file_format: &FileFormat {
        id: 3_055_769_122,
        source_type: SourceType::Iana,
        name: "vnd.cmmf-encoder-configuration+json",
        extensions: &[],
        media_types: &["application/vnd.cmmf-encoder-configuration+json"],
        signatures: &[],
        related_formats: &[],
    },
};
