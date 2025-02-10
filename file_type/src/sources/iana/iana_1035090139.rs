use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1035090139: FileType = FileType {
    file_format: &FileFormat {
        id: 1_035_090_139,
        source_type: SourceType::Iana,
        name: "rdap+json",
        extensions: &[],
        media_types: &["application/rdap+json"],
        signatures: &[],
        related_formats: &[],
    },
};
