use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2903592947: FileType = FileType {
    file_format: &FileFormat {
        id: 2_903_592_947,
        source_type: SourceType::Iana,
        name: "did",
        extensions: &[],
        media_types: &["application/did"],
        signatures: &[],
        related_formats: &[],
    },
};
