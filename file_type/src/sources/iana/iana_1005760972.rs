use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1005760972: FileType = FileType {
    file_format: &FileFormat {
        id: 1_005_760_972,
        source_type: SourceType::Iana,
        name: "vnd.epson.msf",
        extensions: &[],
        media_types: &["application/vnd.epson.msf"],
        signatures: &[],
        related_formats: &[],
    },
};
