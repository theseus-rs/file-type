use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3150059966: FileType = FileType {
    file_format: &FileFormat {
        id: 3_150_059_966,
        source_type: SourceType::Iana,
        name: "gif",
        extensions: &[],
        media_types: &["image/gif"],
        signatures: &[],
        related_formats: &[],
    },
};
