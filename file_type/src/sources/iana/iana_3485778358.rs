use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3485778358: FileType = FileType {
    file_format: &FileFormat {
        id: 3_485_778_358,
        source_type: SourceType::Iana,
        name: "vnd.hp-PCLXL",
        extensions: &[],
        media_types: &["application/vnd.hp-PCLXL"],
        signatures: &[],
        related_formats: &[],
    },
};
