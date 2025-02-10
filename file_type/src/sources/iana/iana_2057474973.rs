use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2057474973: FileType = FileType {
    file_format: &FileFormat {
        id: 2_057_474_973,
        source_type: SourceType::Iana,
        name: "g3fax",
        extensions: &[],
        media_types: &["image/g3fax"],
        signatures: &[],
        related_formats: &[],
    },
};
