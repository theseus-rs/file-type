use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1855357811: FileType = FileType {
    file_format: &FileFormat {
        id: 1_855_357_811,
        source_type: SourceType::Iana,
        name: "nlsml+xml",
        extensions: &[],
        media_types: &["application/nlsml+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
