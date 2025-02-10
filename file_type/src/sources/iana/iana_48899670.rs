use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_48899670: FileType = FileType {
    file_format: &FileFormat {
        id: 48_899_670,
        source_type: SourceType::Iana,
        name: "jls",
        extensions: &[],
        media_types: &["image/jls"],
        signatures: &[],
        related_formats: &[],
    },
};
