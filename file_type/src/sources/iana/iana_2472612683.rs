use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2472612683: FileType = FileType {
    file_format: &FileFormat {
        id: 2_472_612_683,
        source_type: SourceType::Iana,
        name: "vnd.accpac.simply.aso",
        extensions: &[],
        media_types: &["application/vnd.accpac.simply.aso"],
        signatures: &[],
        related_formats: &[],
    },
};
