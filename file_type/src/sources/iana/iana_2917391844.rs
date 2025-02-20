use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2917391844: FileType = FileType {
    file_format: &FileFormat {
        id: 2_917_391_844,
        source_type: SourceType::Iana,
        name: "stratum",
        extensions: &[],
        media_types: &["application/stratum"],
        signatures: &[],
        related_formats: &[],
    },
};
