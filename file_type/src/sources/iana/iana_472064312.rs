use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_472064312: FileType = FileType {
    file_format: &FileFormat {
        id: 472_064_312,
        source_type: SourceType::Iana,
        name: "cccex",
        extensions: &[],
        media_types: &["application/cccex"],
        signatures: &[],
        related_formats: &[],
    },
};
