use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2917306439: FileType = FileType {
    file_format: &FileFormat {
        id: 2_917_306_439,
        source_type: SourceType::Iana,
        name: "global-headers",
        extensions: &[],
        media_types: &["message/global-headers"],
        signatures: &[],
        related_formats: &[],
    },
};
