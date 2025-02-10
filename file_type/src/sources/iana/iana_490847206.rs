use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_490847206: FileType = FileType {
    file_format: &FileFormat {
        id: 490_847_206,
        source_type: SourceType::Iana,
        name: "cose-key-set",
        extensions: &[],
        media_types: &["application/cose-key-set"],
        signatures: &[],
        related_formats: &[],
    },
};
