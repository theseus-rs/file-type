use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3473343428: FileType = FileType {
    file_format: &FileFormat {
        id: 3_473_343_428,
        source_type: SourceType::Iana,
        name: "naplps",
        extensions: &[],
        media_types: &["image/naplps"],
        signatures: &[],
        related_formats: &[],
    },
};
