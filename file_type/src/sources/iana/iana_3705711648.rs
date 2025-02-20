use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3705711648: FileType = FileType {
    file_format: &FileFormat {
        id: 3_705_711_648,
        source_type: SourceType::Iana,
        name: "vnd.epson.ssf",
        extensions: &[],
        media_types: &["application/vnd.epson.ssf"],
        signatures: &[],
        related_formats: &[],
    },
};
