use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_330977383: FileType = FileType {
    file_format: &FileFormat {
        id: 330_977_383,
        source_type: SourceType::Iana,
        name: "encaprtp",
        extensions: &[],
        media_types: &["application/encaprtp"],
        signatures: &[],
        related_formats: &[],
    },
};
