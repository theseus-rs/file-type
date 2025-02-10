use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1550746427: FileType = FileType {
    file_format: &FileFormat {
        id: 1_550_746_427,
        source_type: SourceType::Iana,
        name: "pkixcmp",
        extensions: &[],
        media_types: &["application/pkixcmp"],
        signatures: &[],
        related_formats: &[],
    },
};
