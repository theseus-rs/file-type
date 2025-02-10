use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1050342415: FileType = FileType {
    file_format: &FileFormat {
        id: 1_050_342_415,
        source_type: SourceType::Iana,
        name: "vcard",
        extensions: &[],
        media_types: &["text/vcard"],
        signatures: &[],
        related_formats: &[],
    },
};
