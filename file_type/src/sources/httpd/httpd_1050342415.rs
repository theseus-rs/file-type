use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_1050342415: FileType = FileType {
    file_format: &FileFormat {
        id: 1_050_342_415,
        source_type: SourceType::Httpd,
        name: "vcard",
        extensions: &["vcard"],
        media_types: &["text/vcard"],
        signatures: &[],
        related_formats: &[],
    },
};
